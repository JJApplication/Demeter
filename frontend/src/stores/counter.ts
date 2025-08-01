import { ref } from 'vue'
import { defineStore } from 'pinia'
import axios from 'axios'
import { useAuthStore } from './auth'

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

export interface HistoryDay {
  date: string
  count: number
  completed_count: number
  tasks: Todo[]
}

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
