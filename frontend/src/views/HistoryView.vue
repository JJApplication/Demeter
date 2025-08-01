<template>
  <div class="history-container">
    <header class="history-header">
      <div class="header-content">
        <h1>📊 任务历史</h1>
        <div class="header-actions">
          <router-link to="/" class="back-btn">
            ← 返回任务
          </router-link>
          <router-link v-if="!authStore.isAuthenticated" to="/login" class="login-btn">
            🔑 登录
          </router-link>
          <!-- 浮动按钮移到页面底部 -->
        </div>
      </div>
    </header>

    <main class="history-main">
      <div v-if="!authStore.isAuthenticated" class="login-prompt">
        <div class="prompt-icon">🔐</div>
        <h3>登录后可以查看您的任务历史数据</h3>
        <p>包括任务统计、活动热力图等个人数据分析</p>
        <router-link to="/login" class="prompt-login-btn">
          🔑 立即登录
        </router-link>
      </div>
      
      <div v-else class="stats-section">
        <div class="stats-grid">
          <div class="stat-card">
            <div class="stat-icon">📝</div>
            <div class="stat-content">
              <h3>{{ totalTasks }}</h3>
              <p>总任务数</p>
            </div>
          </div>
          <div class="stat-card">
            <div class="stat-icon">✅</div>
            <div class="stat-content">
              <h3>{{ completedTasks }}</h3>
              <p>已完成</p>
            </div>
          </div>
          <div class="stat-card">
            <div class="stat-icon">🔥</div>
            <div class="stat-content">
              <h3>{{ streakDays }}</h3>
              <p>连续天数</p>
            </div>
          </div>
          <div class="stat-card">
            <div class="stat-icon">📈</div>
            <div class="stat-content">
              <h3>{{ completionRate }}%</h3>
              <p>完成率</p>
            </div>
          </div>
        </div>
      </div>

      <div v-if="authStore.isAuthenticated" class="activity-section">
        <div class="activity-header">
          <h2>📅 活动热力图</h2>
          <p>{{ activityData.length }} 天的任务活动记录</p>
        </div>
        
        <div class="activity-chart">
          <div class="months-labels">
            <span v-for="month in monthLabels" :key="month" class="month-label">
              {{ month }}
            </span>
          </div>
          
          <div class="chart-container">
            <div class="weekdays-labels">
              <span class="weekday-label">周一</span>
              <span class="weekday-label"></span>
              <span class="weekday-label">周三</span>
              <span class="weekday-label"></span>
              <span class="weekday-label">周五</span>
              <span class="weekday-label"></span>
              <span class="weekday-label">周日</span>
            </div>
            
            <div class="activity-grid">
              <div
                v-for="(day, index) in activityData"
                :key="index"
                :class="['activity-day', getActivityLevel(day.count)]"
                :title="`${day.date}: ${day.count} 个任务`"
                @click="showDayDetails(day)"
              >
              </div>
            </div>
          </div>
          
          <div class="legend">
            <span class="legend-text">少</span>
            <div class="legend-colors">
              <div class="legend-color level-0"></div>
              <div class="legend-color level-1"></div>
              <div class="legend-color level-2"></div>
              <div class="legend-color level-3"></div>
              <div class="legend-color level-4"></div>
            </div>
            <span class="legend-text">多</span>
          </div>
        </div>
      </div>

      <!-- 日期详情弹窗 -->
      <div v-if="selectedDay" class="modal-overlay" @click="closeModal">
        <div class="modal-content" @click.stop>
          <div class="modal-header">
            <h3>{{ selectedDay.date }}</h3>
            <button @click="closeModal" class="close-btn">×</button>
          </div>
          <div class="modal-body">
            <div v-if="selectedDay.tasks.length === 0" class="no-tasks">
              <p>这一天没有任务记录</p>
            </div>
            <div v-else class="tasks-list">
              <div
                v-for="task in selectedDay.tasks"
                :key="task.id"
                :class="['task-item', { completed: task.completed }]"
              >
                <span class="task-emoji">{{ task.emoji }}</span>
                <span class="task-title">{{ task.title }}</span>
                <span class="task-status">{{ task.completed ? '✅' : '⭕' }}</span>
              </div>
            </div>
          </div>
        </div>
      </div>
      
      <!-- 浮动按钮 -->
      <div v-if="authStore.isAuthenticated" class="floating-toggle">
        <label class="floating-toggle-btn">
          <input 
            type="checkbox" 
            v-model="publicAccessEnabled" 
            @change="togglePublicAccess"
            class="toggle-checkbox"
          >
          <div class="toggle-icon">
            <span v-if="publicAccessEnabled">🌐</span>
            <span v-else>🔒</span>
          </div>
        </label>
      </div>
    </main>
  </div>
</template>

<script setup lang="ts">
import { ref, onMounted, computed } from 'vue'
import { useTodoStore } from '../stores/counter'
import { useAuthStore } from '../stores/auth'

interface ActivityDay {
  date: string
  count: number
  tasks: Array<{
    id: number
    title: string
    emoji: string
    completed: boolean
  }>
}

const todoStore = useTodoStore()
const authStore = useAuthStore()
const activityData = ref<ActivityDay[]>([])
const selectedDay = ref<ActivityDay | null>(null)
const publicAccessEnabled = ref(false)

const totalTasks = computed(() => {
  return activityData.value.reduce((sum, day) => sum + day.count, 0)
})

const completedTasks = computed(() => {
  return activityData.value.reduce((sum, day) => {
    return sum + day.tasks.filter(task => task.completed).length
  }, 0)
})

const completionRate = computed(() => {
  if (totalTasks.value === 0) return 0
  return Math.round((completedTasks.value / totalTasks.value) * 100)
})

const streakDays = computed(() => {
  let streak = 0
  for (let i = activityData.value.length - 1; i >= 0; i--) {
    if (activityData.value[i].count > 0) {
      streak++
    } else {
      break
    }
  }
  return streak
})

const monthLabels = computed(() => {
  const months = ['1月', '2月', '3月', '4月', '5月', '6月', '7月', '8月', '9月', '10月', '11月', '12月']
  const result = []
  const now = new Date()
  
  for (let i = 11; i >= 0; i--) {
    const date = new Date(now.getFullYear(), now.getMonth() - i, 1)
    result.push(months[date.getMonth()])
  }
  
  return result
})

const generateActivityData = () => {
  const data: ActivityDay[] = []
  const now = new Date()
  
  // 生成过去365天的数据
  for (let i = 364; i >= 0; i--) {
    const date = new Date(now)
    date.setDate(date.getDate() - i)
    
    const dateString = date.toISOString().split('T')[0]
    const count = Math.floor(Math.random() * 8) // 0-7个任务
    const tasks = []
    
    // 生成模拟任务数据
    for (let j = 0; j < count; j++) {
      tasks.push({
        id: Date.now() + j,
        title: `任务 ${j + 1}`,
        emoji: ['📝', '💼', '🎯', '⚡', '🔥'][Math.floor(Math.random() * 5)],
        completed: Math.random() > 0.3
      })
    }
    
    data.push({
      date: dateString,
      count,
      tasks
    })
  }
  
  return data
}

const getActivityLevel = (count: number): string => {
  if (count === 0) return 'level-0'
  if (count <= 2) return 'level-1'
  if (count <= 4) return 'level-2'
  if (count <= 6) return 'level-3'
  return 'level-4'
}

const showDayDetails = (day: ActivityDay) => {
  selectedDay.value = day
}

const closeModal = () => {
  selectedDay.value = null
}

const togglePublicAccess = async () => {
  const success = await authStore.updateUserSettings({
    public_access: publicAccessEnabled.value
  })
  
  if (!success) {
    // 如果更新失败，恢复原状态
    publicAccessEnabled.value = !publicAccessEnabled.value
    alert('更新设置失败，请重试')
  }
}

onMounted(async () => {
  // 获取公开访问状态
  await authStore.fetchPublicAccess()
  if (authStore.publicAccess) {
    publicAccessEnabled.value = authStore.publicAccess.public_access
  }
  
  // 获取历史数据
  if (authStore.isAuthenticated || authStore.publicAccess?.public_access) {
    try {
      const historyData = await todoStore.getHistoryData()
      activityData.value = historyData.map((day: any) => ({
        date: day.date,
        count: day.count,
        tasks: day.tasks.map((task: any) => ({
          id: task.id,
          title: task.title,
          emoji: task.emoji,
          completed: task.completed
        }))
      }))
    } catch (error) {
      console.error('获取历史数据失败:', error)
      // 如果获取失败，使用模拟数据作为后备
      activityData.value = generateActivityData()
    }
  }
})
</script>

<style scoped>
.history-container {
  min-height: 100vh;
  background: linear-gradient(135deg, #f5f7fa 0%, #c3cfe2 100%);
}

.history-header {
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

.back-btn, .login-btn {
  padding: 10px 20px;
  color: white;
  text-decoration: none;
  border-radius: 10px;
  font-weight: 500;
  transition: all 0.3s ease;
  display: inline-flex;
  align-items: center;
  gap: 8px;
  margin-left: 10px;
}

.back-btn {
  background: #667eea;
}

.login-btn {
  background: linear-gradient(135deg, #4CAF50, #45a049);
}

.back-btn:hover {
  transform: translateY(-2px);
  box-shadow: 0 5px 15px rgba(102, 126, 234, 0.3);
}

.login-btn:hover {
  background: linear-gradient(135deg, #45a049, #3d8b40);
  transform: translateY(-2px);
  box-shadow: 0 4px 12px rgba(76, 175, 80, 0.3);
}

.login-prompt {
  text-align: center;
  background: white;
  border-radius: 20px;
  padding: 60px 40px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.1);
  margin-bottom: 40px;
}

.prompt-icon {
  font-size: 4rem;
  margin-bottom: 20px;
}

.login-prompt h3 {
  color: #333;
  margin-bottom: 15px;
  font-size: 1.5rem;
}

.login-prompt p {
  color: #666;
  margin-bottom: 30px;
  font-size: 1rem;
  line-height: 1.6;
}

.prompt-login-btn {
  background: linear-gradient(135deg, #4CAF50, #45a049);
  color: white;
  text-decoration: none;
  padding: 15px 30px;
  border-radius: 12px;
  font-weight: 600;
  font-size: 1.1rem;
  transition: all 0.3s ease;
  display: inline-flex;
  align-items: center;
  gap: 10px;
}

.prompt-login-btn:hover {
  background: linear-gradient(135deg, #45a049, #3d8b40);
  transform: translateY(-3px);
  box-shadow: 0 8px 20px rgba(76, 175, 80, 0.3);
}

.settings-section {
  margin-left: 15px;
}

.public-access-toggle {
  display: flex;
  align-items: center;
  gap: 12px;
  cursor: pointer;
  padding: 8px 12px;
  background: rgba(255, 255, 255, 0.1);
  border-radius: 8px;
  transition: all 0.3s ease;
}

.public-access-toggle:hover {
  background: rgba(255, 255, 255, 0.2);
}

.toggle-checkbox {
  position: relative;
  width: 44px;
  height: 24px;
  appearance: none;
  background: rgba(255, 255, 255, 0.3);
  border-radius: 12px;
  cursor: pointer;
  transition: all 0.3s ease;
  outline: none;
}

.toggle-checkbox:checked {
  background: #4caf50;
}

.toggle-checkbox::before {
  content: '';
  position: absolute;
  top: 2px;
  left: 2px;
  width: 20px;
  height: 20px;
  background: white;
  border-radius: 50%;
  transition: all 0.3s ease;
  box-shadow: 0 2px 4px rgba(0, 0, 0, 0.2);
}

.toggle-checkbox:checked::before {
  transform: translateX(20px);
}

.toggle-label {
  color: rgb(15, 128, 58);
  font-size: 0.9rem;
  font-weight: 500;
  user-select: none;
}

.history-main {
  max-width: 1200px;
  margin: 0 auto;
  padding: 30px 20px;
}

.stats-section {
  margin-bottom: 40px;
}

.stats-grid {
  display: grid;
  grid-template-columns: repeat(auto-fit, minmax(200px, 1fr));
  gap: 20px;
}

.stat-card {
  background: white;
  border-radius: 15px;
  padding: 25px;
  box-shadow: 0 5px 15px rgba(0, 0, 0, 0.08);
  display: flex;
  align-items: center;
  gap: 15px;
  transition: all 0.3s ease;
}

.stat-card:hover {
  transform: translateY(-3px);
  box-shadow: 0 10px 25px rgba(0, 0, 0, 0.15);
}

.stat-icon {
  font-size: 2.5rem;
  flex-shrink: 0;
}

.stat-content h3 {
  margin: 0 0 5px 0;
  font-size: 2rem;
  color: #333;
}

.stat-content p {
  margin: 0;
  color: #666;
  font-size: 0.9rem;
}

.activity-section {
  background: white;
  border-radius: 20px;
  padding: 30px;
  box-shadow: 0 10px 30px rgba(0, 0, 0, 0.1);
}

.activity-header {
  margin-bottom: 30px;
}

.activity-header h2 {
  margin: 0 0 5px 0;
  color: #333;
  font-size: 1.5rem;
}

.activity-header p {
  margin: 0;
  color: #666;
  font-size: 0.9rem;
}

.activity-chart {
  overflow-x: auto;
}

.months-labels {
  display: flex;
  margin-bottom: 10px;
  padding-left: 60px;
}

.month-label {
  flex: 1;
  text-align: center;
  font-size: 0.8rem;
  color: #666;
  min-width: 60px;
}

.chart-container {
  display: flex;
  gap: 10px;
}

.weekdays-labels {
  display: flex;
  flex-direction: column;
  gap: 2px;
  width: 50px;
}

.weekday-label {
  height: 12px;
  font-size: 0.7rem;
  color: #666;
  display: flex;
  align-items: center;
  margin-bottom: 1px;
}

.activity-grid {
  display: grid;
  grid-template-columns: repeat(53, 12px);
  grid-template-rows: repeat(7, 12px);
  gap: 2px;
  grid-auto-flow: column;
}

.activity-day {
  width: 12px;
  height: 12px;
  border-radius: 2px;
  cursor: pointer;
  transition: all 0.2s ease;
}

.activity-day:hover {
  transform: scale(1.2);
  border: 1px solid #333;
}

.level-0 {
  background-color: #ebedf0;
}

.level-1 {
  background-color: #9be9a8;
}

.level-2 {
  background-color: #40c463;
}

.level-3 {
  background-color: #30a14e;
}

.level-4 {
  background-color: #216e39;
}

.legend {
  display: flex;
  align-items: center;
  justify-content: flex-end;
  gap: 5px;
  margin-top: 15px;
  font-size: 0.8rem;
  color: #666;
}

.legend-colors {
  display: flex;
  gap: 2px;
}

.legend-color {
  width: 12px;
  height: 12px;
  border-radius: 2px;
}

.legend-text {
  font-size: 0.75rem;
}

/* 模态框样式 */
.modal-overlay {
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

@keyframes fadeIn {
  from { opacity: 0; }
  to { opacity: 1; }
}

.modal-content {
  background: white;
  border-radius: 15px;
  max-width: 500px;
  width: 90%;
  max-height: 80vh;
  overflow-y: auto;
  animation: slideUp 0.3s ease;
}

@keyframes slideUp {
  from {
    opacity: 0;
    transform: translateY(30px);
  }
  to {
    opacity: 1;
    transform: translateY(0);
  }
}

.modal-header {
  padding: 20px 25px;
  border-bottom: 1px solid #eee;
  display: flex;
  justify-content: space-between;
  align-items: center;
}

.modal-header h3 {
  margin: 0;
  color: #333;
}

.close-btn {
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

.close-btn:hover {
  background: #f5f5f5;
  color: #333;
}

.modal-body {
  padding: 25px;
}

.no-tasks {
  text-align: center;
  color: #666;
  padding: 20px;
}

.tasks-list {
  display: flex;
  flex-direction: column;
  gap: 10px;
}

.task-item {
  display: flex;
  align-items: center;
  gap: 12px;
  padding: 12px;
  background: #f8f9fa;
  border-radius: 10px;
  transition: all 0.2s ease;
}

.task-item:hover {
  background: #e9ecef;
}

.task-item.completed {
  opacity: 0.7;
}

.task-emoji {
  font-size: 1.2rem;
  flex-shrink: 0;
}

.task-title {
  flex: 1;
  color: #333;
}

.task-item.completed .task-title {
  text-decoration: line-through;
  color: #999;
}

.task-status {
  font-size: 1.1rem;
  flex-shrink: 0;
}

@media (max-width: 768px) {
  .header-content {
    flex-direction: column;
    gap: 15px;
  }
  
  .stats-grid {
    grid-template-columns: repeat(2, 1fr);
  }
  
  .activity-chart {
    font-size: 0.8rem;
  }
}

/* 浮动按钮样式 */
.floating-toggle {
  position: fixed;
  bottom: 30px;
  right: 30px;
  z-index: 1000;
}

.floating-toggle-btn {
  display: block;
  cursor: pointer;
}

.floating-toggle .toggle-checkbox {
  display: none;
}

.toggle-icon {
  width: 60px;
  height: 60px;
  background: linear-gradient(135deg, #667eea, #764ba2);
  border-radius: 50%;
  display: flex;
  align-items: center;
  justify-content: center;
  font-size: 24px;
  color: white;
  box-shadow: 0 4px 20px rgba(0, 0, 0, 0.15);
  transition: all 0.3s ease;
  cursor: pointer;
}

.toggle-icon:hover {
  transform: translateY(-2px);
  box-shadow: 0 6px 25px rgba(0, 0, 0, 0.2);
}

.floating-toggle .toggle-checkbox:checked + .toggle-icon {
  background: linear-gradient(135deg, #4c90af, #0caabf);
}

@media (max-width: 768px) {
  .floating-toggle {
    bottom: 20px;
    right: 20px;
  }
  
  .toggle-icon {
    width: 50px;
    height: 50px;
    font-size: 20px;
  }
}
  
  .activity-grid {
    grid-template-columns: repeat(26, 10px);
    grid-template-rows: repeat(7, 10px);
  }
  
  .activity-day {
    width: 10px;
    height: 10px;
  }
  
  .weekdays-labels {
    width: 40px;
  }

</style>