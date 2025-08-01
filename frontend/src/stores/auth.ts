import { ref, computed } from 'vue'
import { defineStore } from 'pinia'
import axios from 'axios'

const API_BASE = '/api'

export interface User {
  id: number
  username: string
  email?: string
  publicAccess?: boolean
}

export interface PublicAccessResponse {
  public_access: boolean
  username: string
}

export const useAuthStore = defineStore('auth', () => {
  const user = ref<User | null>(null)
  const isAuthenticated = computed(() => !!user.value)
  const token = ref<string | null>(null)
  const publicAccess = ref<PublicAccessResponse | null>(null)

  // 从localStorage恢复用户状态
  const savedUser = localStorage.getItem('user')
  const savedToken = localStorage.getItem('token')
  
  if (savedUser && savedToken) {
    try {
      user.value = JSON.parse(savedUser)
      token.value = savedToken
    } catch (error) {
      console.error('Failed to parse saved user data:', error)
      localStorage.removeItem('user')
      localStorage.removeItem('token')
    }
  }

  const login = async (username: string, password: string) => {
    try {
      const response = await axios.post(`${API_BASE}/login`, { username, password })
      user.value = response.data.user
      token.value = response.data.token
      localStorage.setItem('user', JSON.stringify(user.value))
      localStorage.setItem('token', response.data.token)
      return true
    } catch (error) {
      console.error('Login failed:', error)
      return false
    }
  }

  const logout = () => {
    user.value = null
    token.value = null
    localStorage.removeItem('user')
    localStorage.removeItem('token')
  }

  const initAuth = () => {
    const savedUser = localStorage.getItem('user')
    if (savedUser) {
      user.value = JSON.parse(savedUser)
    }
  }

  const fetchPublicAccess = async () => {
    try {
      const response = await axios.get(`${API_BASE}/public-access`)
      publicAccess.value = response.data
    } catch (error) {
      console.error('Failed to fetch public access status:', error)
    }
  }

  const getAuthHeaders = () => {
    if (!token.value) return {}
    return {
      'Authorization': `Bearer ${token.value}`
    }
  }

  const updateUserSettings = async (settings: { public_access: boolean }) => {
    try {
      const response = await axios.put(`${API_BASE}/user/settings`, settings, {
        headers: {
          'Content-Type': 'application/json',
          ...getAuthHeaders()
        }
      })
      
      if (response.status === 200) {
        const result = response.data
        if (user.value) {
          user.value.publicAccess = result.data.public_access
          localStorage.setItem('user', JSON.stringify(user.value))
        }
        await fetchPublicAccess() // 刷新公开访问状态
        return true
      }
      return false
    } catch (error) {
      console.error('Failed to update user settings:', error)
      return false
    }
  }

  return { 
    user, 
    isAuthenticated, 
    publicAccess, 
    token, 
    login, 
    logout, 
    initAuth, 
    fetchPublicAccess, 
    updateUserSettings, 
    getAuthHeaders 
  }
})