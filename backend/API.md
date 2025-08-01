# 📚 TodoList API 接口文档

## 基础信息

- **Base URL**: `http://localhost:3001/api`
- **Content-Type**: `application/json`
- **认证方式**: 简单用户名密码认证

## 📋 接口列表

### 🔐 用户认证

#### 1. 用户登录

**POST** `/login`

登录系统获取用户信息。

**请求体**:
```json
{
  "username": "guest",
  "password": "password"
}
```

**响应**:
```json
{
  "user": {
    "id": 1,
    "username": "guest"
  },
  "message": "登录成功"
}
```

**状态码**:
- `200` - 登录成功
- `401` - 用户名或密码错误
- `500` - 服务器内部错误

---

#### 2. 用户注册

**POST** `/register`

注册新用户账户。

**请求体**:
```json
{
  "username": "newuser",
  "password": "newpassword"
}
```

**响应**:
```json
{
  "success": true,
  "data": {
    "id": 2,
    "username": "newuser"
  },
  "message": "操作成功"
}
```

**状态码**:
- `200` - 注册成功
- `400` - 用户名已存在
- `500` - 服务器内部错误

---

### 📝 任务管理

#### 3. 获取任务列表

**GET** `/todos`

获取当前用户的所有任务。

**响应**:
```json
[
  {
    "id": 1,
    "title": "完成项目文档",
    "emoji": "📝",
    "completed": false,
    "created_at": "2024-01-15T10:30:00Z",
    "updated_at": "2024-01-15T10:30:00Z"
  },
  {
    "id": 2,
    "title": "学习Rust编程",
    "emoji": "🦀",
    "completed": true,
    "created_at": "2024-01-14T09:15:00Z",
    "updated_at": "2024-01-15T14:20:00Z"
  }
]
```

**状态码**:
- `200` - 获取成功
- `500` - 服务器内部错误

---

#### 4. 创建新任务

**POST** `/todos`

创建一个新的任务。

**请求体**:
```json
{
  "title": "学习Vue3",
  "emoji": "💡"
}
```

**响应**:
```json
{
  "id": 3,
  "title": "学习Vue3",
  "emoji": "💡",
  "completed": false,
  "created_at": "2024-01-15T15:45:00Z",
  "updated_at": "2024-01-15T15:45:00Z"
}
```

**状态码**:
- `200` - 创建成功
- `400` - 请求参数错误
- `500` - 服务器内部错误

---

#### 5. 更新任务

**PUT** `/todos/{id}`

更新指定ID的任务。

**路径参数**:
- `id` (integer) - 任务ID

**请求体** (所有字段都是可选的):
```json
{
  "title": "更新后的任务标题",
  "emoji": "✅",
  "completed": true
}
```

**响应**:
```json
{
  "id": 1,
  "title": "更新后的任务标题",
  "emoji": "✅",
  "completed": true,
  "created_at": "2024-01-15T10:30:00Z",
  "updated_at": "2024-01-15T16:00:00Z"
}
```

**状态码**:
- `200` - 更新成功
- `404` - 任务不存在
- `400` - 请求参数错误
- `500` - 服务器内部错误

---

#### 6. 删除任务

**DELETE** `/todos/{id}`

删除指定ID的任务。

**路径参数**:
- `id` (integer) - 任务ID

**响应**: 无内容

**状态码**:
- `204` - 删除成功
- `404` - 任务不存在
- `500` - 服务器内部错误

---

### 📊 历史数据

#### 7. 获取历史活动数据

**GET** `/todos/history`

获取过去365天的任务活动数据，用于生成活动热力图。

**响应**:
```json
[
  {
    "date": "2024-01-15",
    "count": 3,
    "completed_count": 2,
    "tasks": [
      {
        "id": 1,
        "title": "完成项目文档",
        "emoji": "📝",
        "completed": true,
        "created_at": "2024-01-15T10:30:00Z",
        "updated_at": "2024-01-15T16:00:00Z"
      },
      {
        "id": 2,
        "title": "学习Rust编程",
        "emoji": "🦀",
        "completed": true,
        "created_at": "2024-01-15T09:15:00Z",
        "updated_at": "2024-01-15T14:20:00Z"
      },
      {
        "id": 3,
        "title": "准备会议材料",
        "emoji": "📋",
        "completed": false,
        "created_at": "2024-01-15T15:45:00Z",
        "updated_at": "2024-01-15T15:45:00Z"
      }
    ]
  },
  {
    "date": "2024-01-14",
    "count": 1,
    "completed_count": 1,
    "tasks": [
      {
        "id": 4,
        "title": "代码审查",
        "emoji": "🔍",
        "completed": true,
        "created_at": "2024-01-14T11:20:00Z",
        "updated_at": "2024-01-14T17:30:00Z"
      }
    ]
  }
]
```

**状态码**:
- `200` - 获取成功
- `500` - 服务器内部错误

---

## 📝 数据模型

### User (用户)
```typescript
interface User {
  id: number;           // 用户ID
  username: string;     // 用户名
}
```

### Todo (任务)
```typescript
interface Todo {
  id: number;           // 任务ID
  title: string;        // 任务标题
  emoji: string;        // 任务emoji图标
  completed: boolean;   // 是否完成
  created_at: string;   // 创建时间 (ISO 8601)
  updated_at: string;   // 更新时间 (ISO 8601)
}
```

### HistoryDay (历史数据)
```typescript
interface HistoryDay {
  date: string;         // 日期 (YYYY-MM-DD)
  count: number;        // 当日任务总数
  completed_count: number; // 当日完成任务数
  tasks: Todo[];        // 当日任务列表
}
```

---

## 🚨 错误处理

### 通用错误响应格式
```json
{
  "success": false,
  "data": null,
  "message": "错误描述信息"
}
```

### 常见错误码
- `400 Bad Request` - 请求参数错误
- `401 Unauthorized` - 认证失败
- `404 Not Found` - 资源不存在
- `500 Internal Server Error` - 服务器内部错误

---

## 🔧 使用示例

### JavaScript/Axios 示例

```javascript
// 登录
const login = async (username, password) => {
  try {
    const response = await axios.post('/api/login', {
      username,
      password
    });
    return response.data;
  } catch (error) {
    console.error('登录失败:', error.response.data);
  }
};

// 获取任务列表
const getTodos = async () => {
  try {
    const response = await axios.get('/api/todos');
    return response.data;
  } catch (error) {
    console.error('获取任务失败:', error.response.data);
  }
};

// 创建任务
const createTodo = async (title, emoji) => {
  try {
    const response = await axios.post('/api/todos', {
      title,
      emoji
    });
    return response.data;
  } catch (error) {
    console.error('创建任务失败:', error.response.data);
  }
};

// 更新任务状态
const toggleTodo = async (id, completed) => {
  try {
    const response = await axios.put(`/api/todos/${id}`, {
      completed
    });
    return response.data;
  } catch (error) {
    console.error('更新任务失败:', error.response.data);
  }
};
```

### cURL 示例

```bash
# 登录
curl -X POST http://localhost:3000/api/login \
  -H "Content-Type: application/json" \
  -d '{"username":"guest","password":"password"}'

# 获取任务列表
curl -X GET http://localhost:3000/api/todos

# 创建新任务
curl -X POST http://localhost:3000/api/todos \
  -H "Content-Type: application/json" \
  -d '{"title":"新任务","emoji":"📝"}'

# 更新任务
curl -X PUT http://localhost:3000/api/todos/1 \
  -H "Content-Type: application/json" \
  -d '{"completed":true}'

# 删除任务
curl -X DELETE http://localhost:3000/api/todos/1
```

---

## 📋 注意事项

1. **时间格式**: 所有时间字段使用 ISO 8601 格式 (UTC)
2. **Emoji支持**: 支持所有Unicode emoji字符
3. **数据库**: 使用SQLite，数据存储在 `todolist.db` 文件中
4. **CORS**: 已配置允许前端域名访问
5. **默认用户**: 系统会自动创建默认用户 `guest/password`

---

## 🔄 版本信息

- **API版本**: v1.0.0
- **最后更新**: 2024-01-15
- **维护者**: TodoList Team