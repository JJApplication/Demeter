import { ref, computed } from 'vue'
import { defineStore } from 'pinia'
import axios from 'axios'

const API_BASE = 'http://localhost:9999/api'

export interface Todo {
  id: number
  title: string
  description?: string
  emoji: string
  completed: boolean
  created_at: string
  updated_at: string
}

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

export interface HistoryDay {
  date: string
  count: number
  completed_count: number
  tasks: Todo[]
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
      const response = await fetch('http://localhost:9999/api/public-access')
      if (response.ok) {
        publicAccess.value = await response.json()
      }
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
      const response = await fetch('http://localhost:9999/api/user/settings', {
        method: 'PUT',
        headers: {
          'Content-Type': 'application/json',
          ...getAuthHeaders()
        },
        body: JSON.stringify(settings)
      })
      
      if (response.ok) {
        const result = await response.json()
        if (user.value) {
          user.value.public_access = result.data.public_access
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
  return { user, isAuthenticated, publicAccess, token, login, logout, initAuth, fetchPublicAccess, updateUserSettings, getAuthHeaders }
})

export const useTodoStore = defineStore('todo', () => {
  const todos = ref<Todo[]>([])
  const loading = ref(false)

  const fetchTodos = async () => {
    loading.value = true
    try {
      const authStore = useAuthStore()
      const headers = authStore.getAuthHeaders()
      const response = await axios.get(`${API_BASE}/todos`, { headers })
      todos.value = response.data
    } catch (error) {
      console.error('Failed to fetch todos:', error)
    } finally {
      loading.value = false
    }
  }

  const addTodo = async (title: string, emoji: string, description?: string) => {
    try {
      const authStore = useAuthStore()
      const headers = authStore.getAuthHeaders()
      const response = await axios.post(`${API_BASE}/todos`, { title, description, emoji }, { headers })
      todos.value.push(response.data)
      return true
    } catch (error) {
      console.error('Failed to add todo:', error)
      return false
    }
  }

  const toggleTodo = async (id: number) => {
    try {
      const todo = todos.value.find(t => t.id === id)
      if (todo) {
        const authStore = useAuthStore()
        const headers = authStore.getAuthHeaders()
        const response = await axios.put(`${API_BASE}/todos/${id}`, {
          completed: !todo.completed
        }, { headers })
        Object.assign(todo, response.data)
      }
    } catch (error) {
      console.error('Failed to toggle todo:', error)
    }
  }

  const deleteTodo = async (id: number) => {
    try {
      const authStore = useAuthStore()
      const headers = authStore.getAuthHeaders()
      await axios.delete(`${API_BASE}/todos/${id}`, { headers })
      todos.value = todos.value.filter(t => t.id !== id)
    } catch (error) {
      console.error('Failed to delete todo:', error)
    }
  }

  const getHistoryData = async () => {
    try {
      const authStore = useAuthStore()
      const headers = authStore.getAuthHeaders()
      const response = await axios.get(`${API_BASE}/history`, { headers })
      return response.data
    } catch (error) {
      console.error('Failed to fetch history:', error)
      return []
    }
  }

  return {
    todos,
    loading,
    fetchTodos,
    addTodo,
    toggleTodo,
    deleteTodo,
    getHistoryData
  }
})
