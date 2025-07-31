<template>
  <div class="todo-container">
    <header class="todo-header">
      <div class="header-content">
        <h1>📝 今日任务</h1>
        <div class="header-actions">
          <router-link to="/history" class="history-btn">
            📊 历史任务
          </router-link>
          <button v-if="authStore.isAuthenticated" @click="logout" class="logout-btn">
            🚪 退出登录
          </button>
          <router-link v-if="!authStore.isAuthenticated" to="/login" class="login-btn">
            🔑 登录
          </router-link>
        </div>
      </div>
    </header>

    <main class="todo-main">
      <!-- 任务创建表单 - 只在登录时显示 -->
      <div v-if="authStore.isAuthenticated" class="add-todo-section">
        <div class="add-todo-card">
          <h3>✨ 创建新任务</h3>
          <form @submit.prevent="addTodo" class="add-todo-form">
            <div class="form-row">
              <div class="input-with-emoji">
                <button
                  type="button"
                  @click="toggleEmojiPicker"
                  class="emoji-trigger-btn"
                >
                  {{ selectedEmoji }}
                </button>
                <input
                  v-model="newTodoTitle"
                  type="text"
                  placeholder="输入任务内容..."
                  required
                  class="todo-input"
                />
              </div>
              <button type="submit" class="add-btn">
                ➕ 添加
              </button>
            </div>
            <div class="form-row">
              <textarea
                v-model="newTodoDescription"
                placeholder="任务描述（可选）..."
                class="todo-description"
                rows="3"
              ></textarea>
            </div>
            
            <!-- Emoji选择卡片 -->
            <div v-if="showEmojiPicker" class="emoji-picker-overlay" @click="closeEmojiPicker">
              <div class="emoji-picker-card" @click.stop>
                <div class="emoji-picker-header">
                  <h4>选择表情</h4>
                  <button @click="closeEmojiPicker" class="close-picker-btn">×</button>
                </div>
                <div class="emoji-grid">
                  <button
                    v-for="emoji in emojis"
                    :key="emoji"
                    type="button"
                    @click="selectEmoji(emoji)"
                    :class="['emoji-option', { active: selectedEmoji === emoji }]"
                  >
                    {{ emoji }}
                  </button>
                </div>
              </div>
            </div>
          </form>
        </div>
      </div>

      <!-- 公开访问提示 -->
      <div v-if="!authStore.isAuthenticated && authStore.publicAccess" class="public-access-info">
        <p>🌐 您正在查看 <strong>{{ authStore.publicAccess.username }}</strong> 的公开任务列表</p>
        <p>要创建和管理任务，请先登录</p>
      </div>

      <div class="todos-section">
        <div v-if="!authStore.isAuthenticated && !authStore.publicAccess" class="login-prompt">
          <div class="prompt-icon">🔐</div>
          <h3>登录后可以创建和管理您的个人任务</h3>
          <router-link to="/login" class="prompt-login-btn">
            🔑 立即登录
          </router-link>
        </div>
        
        <div v-else-if="todoStore.loading" class="loading">
          <div class="spinner"></div>
          <p>加载中...</p>
        </div>
        
        <div v-else-if="todoStore.todos.length === 0" class="empty-state">
          <div class="empty-icon">📋</div>
          <h3>暂无任务</h3>
          <p v-if="authStore.isAuthenticated">创建你的第一个任务吧！</p>
          <p v-else>暂时没有公开的任务</p>
        </div>
        
        <div v-else class="todos-grid">
          <div
            v-for="todo in todoStore.todos"
            :key="todo.id"
            :class="['todo-card', { completed: todo.completed }]"
          >
            <div class="todo-emoji">{{ todo.emoji }}</div>
            <div class="todo-content">
              <h4 :class="{ 'line-through': todo.completed }">{{ todo.title }}</h4>
              <p v-if="todo.description" class="todo-description-text">{{ todo.description }}</p>
              <p class="todo-time">{{ formatTime(todo.created_at) }}</p>
            </div>
            <div v-if="authStore.isAuthenticated" class="todo-actions">
              <button
                @click="toggleTodo(todo.id)"
                :class="['toggle-btn', { completed: todo.completed }]"
              >
                {{ todo.completed ? '✅' : '⭕' }}
              </button>
              <button @click="deleteTodo(todo.id)" class="delete-btn">
                🗑️
              </button>
            </div>
          </div>
        </div>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted } from 'vue'
import { useRouter } from 'vue-router'
import { useAuthStore, useTodoStore } from '../stores/counter'

const router = useRouter()
const authStore = useAuthStore()
const todoStore = useTodoStore()

const newTodoTitle = ref('')
const newTodoDescription = ref('')
const selectedEmoji = ref('🔥')
const showEmojiPicker = ref(false)

const emojis = [
  '📝', '💼', '🎯', '⚡', '🔥', '💡', '🚀', '⭐',
  '🎨', '📚', '💪', '🏃', '🍎', '☕', '🎵', '🌟'
]

const toggleEmojiPicker = () => {
  showEmojiPicker.value = !showEmojiPicker.value
}

const closeEmojiPicker = () => {
  showEmojiPicker.value = false
}

const selectEmoji = (emoji: string) => {
  selectedEmoji.value = emoji
  showEmojiPicker.value = false
}

const addTodo = async () => {
  if (newTodoTitle.value.trim()) {
    const success = await todoStore.addTodo(
      newTodoTitle.value.trim(), 
      selectedEmoji.value,
      newTodoDescription.value.trim()
    )
    if (success) {
      newTodoTitle.value = ''
      newTodoDescription.value = ''
      selectedEmoji.value = '🔥'
    }
  }
}

const toggleTodo = async (id: number) => {
  await todoStore.toggleTodo(id)
}

const deleteTodo = async (id: number) => {
  await todoStore.deleteTodo(id)
}

const logout = () => {
  authStore.logout()
  router.push('/login')
}

const formatTime = (dateString: string) => {
  const date = new Date(dateString)
  return date.toLocaleString('zh-CN', {
    month: 'short',
    day: 'numeric',
    hour: '2-digit',
    minute: '2-digit'
  })
}

onMounted(async () => {
  // 获取公开访问状态
  await authStore.fetchPublicAccess()
  
  // 如果用户已登录或有公开访问权限，则获取任务
  if (authStore.isAuthenticated || authStore.publicAccess?.public_access) {
    todoStore.fetchTodos()
  }
})
</script>

<style scoped>
.todo-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
}

.todo-header {
  background: white;
  box-shadow: 0 2px 10px rgba(0, 0, 0, 0.1);
  padding: 20px 0;
}

.header-content {
  max-width: 1200px;
  margin: 0 auto;
  padding: 0 20px;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.header-content h1 {
  margin: 0;
  color: #333;
  font-size: 2rem;
}

.header-actions {
  display: flex;
  gap: 15px;
}

.history-btn, .login-btn, .logout-btn {
  padding: 10px 20px;
  border: none;
  border-radius: 10px;
  font-weight: 500;
  cursor: pointer;
  transition: all 0.3s ease;
  text-decoration: none;
  display: inline-block;
}

.history-btn {
  background: #667eea;
  color: white;
}

.login-btn {
  background: linear-gradient(135deg, #4CAF50, #45a049);
  color: white;
}

.login-btn:hover {
  background: linear-gradient(135deg, #45a049, #3d8b40);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(76, 175, 80, 0.3);
}

.logout-btn {
  background: #e74c3c;
  color: white;
}

.history-btn:hover, .logout-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.2);
}

.todo-main {
  max-width: 1200px;
  margin: 0 auto;
  padding: 30px 20px;
}

.add-todo-section {
  margin-bottom: 40px;
}

.add-todo-card {
  background: white;
  border-radius: 20px;
  padding: 30px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.1);
  animation: slideIn 0.6s ease-out;
}

@keyframes slideIn {
  from {
    opacity: 0;
    transform: translateY(20px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.add-todo-card h3 {
  margin: 0 0 20px 0;
  color: #333;
  font-size: 1.5rem;
}

.form-row {
  display: flex;
  flex-wrap: wrap;
  gap: 15px;
  align-items: center;
  margin-top: 1rem;
}

.input-with-emoji {
  flex: 1;
  display: flex;
  align-items: center;
  position: relative;
  min-width: 200px;
}

.emoji-trigger-btn {
  width: 48px;
  height: 48px;
  border: 2px solid #e1e5e9;
  border-radius: 10px;
  background: white;
  font-size: 1.5rem;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  justify-content: center;
  margin-right: 10px;
}

.emoji-trigger-btn:hover {
  border-color: #667eea;
  background: #f8f9ff;
  transform: scale(1.05);
}

.todo-input {
  flex: 1;
  padding: 12px 16px;
  border: 2px solid #e1e5e9;
  border-radius: 10px;
  font-size: 1rem;
  transition: all 0.3s ease;
}

.todo-description {
  width: 100%;
  padding: 12px 16px;
  border: 2px solid #e1e5e9;
  border-radius: 10px;
  font-size: 1rem;
  font-family: inherit;
  resize: vertical;
  min-height: 80px;
  transition: all 0.3s ease;
}

.todo-description:focus {
  outline: none;
  border-color: #4CAF50;
  box-shadow: 0 0 0 3px rgba(76, 175, 80, 0.1);
}

.todo-input:focus {
  outline: none;
  border-color: #667eea;
  box-shadow: 0 0 0 3px rgba(102, 126, 234, 0.1);
}

.input-with-emoji:focus-within .emoji-trigger-btn {
  border-color: #667eea;
}

.input-with-emoji:focus-within .todo-input {
  border-color: #667eea;
}

.add-btn {
  padding: 12px 24px;
  background: linear-gradient(135deg, #667eea 0%, #764ba2 100%);
  color: white;
  border: none;
  border-radius: 10px;
  font-weight: 600;
  cursor: pointer;
  transition: all 0.3s ease;
}

.add-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 10px 20px rgba(102, 126, 234, 0.3);
}

/* 表情选择器弹窗样式 */
.emoji-picker-overlay {
  position: fixed;
  top: 0;
  left: 0;
  right: 0;
  bottom: 0;
  background: rgba(0, 0, 0, 0.5);
  display: flex;
  align-items: center;
  justify-content: center;
  z-index: 1000;
  animation: fadeIn 0.3s ease;
}

.emoji-picker-card {
  background: white;
  border-radius: 20px;
  padding: 0;
  box-shadow: 0 20px 40px rgba(0, 0, 0, 0.15);
  max-width: 400px;
  width: 90%;
  animation: slideUp 0.3s ease;
}

.emoji-picker-header {
  padding: 20px 25px;
  border-bottom: 1px solid #e1e5e9;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.emoji-picker-header h4 {
  margin: 0;
  color: #333;
  font-size: 1.2rem;
}

.close-picker-btn {
  background: none;
  border: none;
  font-size: 1.5rem;
  cursor: pointer;
  color: #666;
  padding: 0;
  width: 30px;
  height: 30px;
  display: flex;
  align-items: center;
  justify-content: center;
  border-radius: 50%;
  transition: all 0.2s ease;
}

.close-picker-btn:hover {
  background: #f5f5f5;
  color: #333;
}

.emoji-grid {
  padding: 20px;
  display: grid;
  grid-template-columns: repeat(8, 1fr);
  gap: 10px;
}

.emoji-option {
  width: 40px;
  height: 40px;
  border: 2px solid #e1e5e9;
  border-radius: 10px;
  background: white;
  font-size: 1.3rem;
  cursor: pointer;
  transition: all 0.3s ease;
  display: flex;
  align-items: center;
  justify-content: center;
}

.emoji-option:hover {
  border-color: #667eea;
  background: #f8f9ff;
  transform: scale(1.1);
}

.emoji-option.active {
  border-color: #667eea;
  background: #667eea;
  color: white;
  transform: scale(1.05);
}

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(30px) scale(0.95);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.loading {
  text-align: center;
  padding: 40px;
}

.spinner {
  width: 40px;
  height: 40px;
  border: 4px solid #f3f3f3;
  border-top: 4px solid #667eea;
  border-radius: 50%;
  animation: spin 1s linear infinite;
  margin: 0 auto 20px;
}

@keyframes spin {
  0% { transform: rotate(0deg); }
  100% { transform: rotate(360deg); }
}

.login-prompt {
  text-align: center;
  padding: 60px 20px;
  background: linear-gradient(135deg, #f8f9fa, #e9ecef);
  border-radius: 15px;
  margin: 20px 0;
}

.prompt-icon {
  font-size: 4rem;
  margin-bottom: 20px;
  opacity: 0.8;
}

.prompt-login-btn {
  background: linear-gradient(135deg, #007bff, #0056b3);
  color: white;
  padding: 12px 24px;
  border-radius: 8px;
  text-decoration: none;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  font-weight: 500;
  transition: all 0.3s ease;
  margin-top: 15px;
}

.prompt-login-btn:hover {
  background: linear-gradient(135deg, #0056b3, #004085);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(0, 123, 255, 0.3);
}

.public-access-info {
  background: linear-gradient(135deg, #e3f2fd, #bbdefb);
  border: 2px solid #2196f3;
  border-radius: 12px;
  padding: 20px;
  margin-bottom: 30px;
  text-align: center;
  box-shadow: 0 4px 12px rgba(33, 150, 243, 0.2);
}

.public-access-info p {
  margin: 8px 0;
  color: #1565c0;
  font-size: 1rem;
}

.public-access-info strong {
  color: #0d47a1;
  font-weight: 600;
}

.empty-state {
  text-align: center;
  padding: 60px 20px;
  color: #666;
}

.empty-icon {
  font-size: 4rem;
  margin-bottom: 20px;
  opacity: 0.7;
}
.todos-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(300px, 1fr));
  max-width: 100%;
  gap: 20px;
  margin-top: 20px;
}

@media (min-width: 768px) {
  .todos-grid {
    grid-template-columns: repeat(2, 1fr);
  }
}

.todo-card {
  background: white;
  border-radius: 15px;
  padding: 20px;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.08);
  transition: all 0.3s ease;
  animation: cardSlideIn 0.5s ease-out;
  border-left: 4px solid #667eea;
}

@keyframes cardSlideIn {
  from {
    opacity: 0;
    transform: translateY(20px) scale(0.95);
  }
  to {
    opacity: 1;
    transform: translateY(0) scale(1);
  }
}

.todo-card:hover {
  transform: translateY(-5px);
  box-shadow: 0 15px 30px rgba(0, 0, 0, 0.15);
}

.todo-card.completed {
  opacity: 0.7;
  border-left-color: #27ae60;
}

.todo-card {
  display: flex;
  align-items: center;
  gap: 15px;
}

.todo-emoji {
  font-size: 2rem;
  flex-shrink: 0;
}

.todo-content {
  flex: 1;
}

.todo-content h4 {
  margin: 0 0 5px 0;
  color: #333;
  font-size: 1.1rem;
  transition: all 0.3s ease;
}

.todo-content h4.line-through {
  text-decoration: line-through;
  color: #999;
}

.todo-description-text {
  margin: 0 0 8px 0;
  color: #666;
  font-size: 0.9rem;
  line-height: 1.4;
  word-wrap: break-word;
}

.todo-time {
  margin: 0;
  color: #666;
  font-size: 0.85rem;
}

.todo-actions {
  display: flex;
  gap: 10px;
  flex-shrink: 0;
}

.toggle-btn, .delete-btn {
  width: 40px;
  height: 40px;
  border: none;
  border-radius: 10px;
  cursor: pointer;
  transition: all 0.3s ease;
  font-size: 1.2rem;
}

.toggle-btn {
  background: #f8f9fa;
}

.toggle-btn.completed {
  background: #d4edda;
}

.delete-btn {
  background: #f8f9fa;
}

.toggle-btn:hover, .delete-btn:hover {
  transform: scale(1.1);
}

.delete-btn:hover {
  background: #f5c6cb;
}

@media (max-width: 768px) {
  .header-content {
    flex-direction: column;
    gap: 15px;
  }
  
  .form-row {
    flex-direction: column;
    align-items: stretch;
  }
  
  .input-with-emoji {
    min-width: auto;
  }
  
  .emoji-trigger-btn {
    width: 44px;
    height: 44px;
    font-size: 1.3rem;
  }
  
  .emoji-picker-card {
    max-width: 350px;
  }
  
  .emoji-grid {
    grid-template-columns: repeat(6, 1fr);
    gap: 8px;
  }
  
  .emoji-option {
    width: 36px;
    height: 36px;
    font-size: 1.1rem;
  }
  
  .todos-grid {
    grid-template-columns: 1fr;
  }
}
</style>