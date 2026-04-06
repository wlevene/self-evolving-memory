# Self-Evolving Memory 使用文档

## 🚀 快速开始

### 安装

```bash
cd ~/workspace/self-evolving-memory
cargo build --release
```

### 启动服务器

```bash
# 方式1: 使用 CLI
./target/debug/mem --serve --port 3000

# 方式2: 使用 cargo
cargo run -- --serve --port 3000
```

服务器将在 `http://localhost:3000` 启动。

---

## 📖 CLI 使用

### 基本命令

```bash
# 查看帮助
mem --help

# 创建记忆
mem create "用户喜欢简洁回复" --pool implicit --type preference

# 搜索记忆
mem search "用户偏好" --limit 10

# 列出所有记忆
mem list --pool explicit --limit 20

# 查看单个记忆
mem get <uuid>

# 更新记忆
mem update <uuid> --importance 0.9

# 删除记忆
mem delete <uuid>

# 查看统计
mem stats

# 交互模式 (REPL)
mem interactive
```

### 交互模式

```bash
$ mem interactive
mem> help
mem> create 这是一个测试记忆
mem> search 测试
mem> list
mem> stats
mem> exit
```

---

## 🌐 HTTP API

### 基础端点

| 方法 | 端点 | 说明 |
|------|------|------|
| GET | /health | 健康检查 |
| GET | /stats | 系统统计 |

### 记忆操作

#### 创建记忆
```bash
curl -X POST http://localhost:3000/memories \
  -H "Content-Type: application/json" \
  -d '{
    "content": "用户喜欢简洁回复",
    "pool": "implicit",
    "type": "preference",
    "importance": 0.8,
    "tags": ["user", "communication"]
  }'
```

#### 获取记忆
```bash
curl http://localhost:3000/memories/<uuid>
```

#### 更新记忆
```bash
curl -X PUT http://localhost:3000/memories/<uuid> \
  -H "Content-Type: application/json" \
  -d '{"importance": 0.9}'
```

#### 删除记忆
```bash
curl -X DELETE http://localhost:3000/memories/<uuid>
```

#### 列表记忆
```bash
curl "http://localhost:3000/memories?pool=explicit&limit=10"
```

#### 搜索记忆
```bash
curl "http://localhost:3000/memories/search?query=用户偏好&limit=5"
```

### 链接操作

#### 创建链接
```bash
curl -X POST http://localhost:3000/links \
  -H "Content-Type: application/json" \
  -d '{
    "source_id": "<uuid1>",
    "target_id": "<uuid2>",
    "link_type": "related"
  }'
```

#### 获取记忆的链接
```bash
curl http://localhost:3000/memories/<uuid>/links
```

---

## 🐍 Python SDK

### 安装

```bash
cd sdk/python
pip install -e .
```

### 使用示例

```python
from self_evolving_memory import MemoryClient, MemoryPool, MemoryType

# 创建客户端
client = MemoryClient("http://localhost:3000")

# 创建记忆
memory = client.create({
    "content": "用户喜欢简洁回复",
    "pool": "implicit",
    "type": "preference",
    "tags": ["communication"]
})
print(f"Created: {memory['id']}")

# 搜索记忆
results = client.search("用户偏好", limit=5)
for m in results:
    print(f"- {m['content']}")

# 获取统计
stats = client.stats()
print(f"Total memories: {stats['total_memories']}")

# 创建链接
client.link(source_id, target_id, "related")

# 渐进式检索（需要配合 SpreadingActivation）
# 见高级用法章节
```

---

## 📦 TypeScript SDK

### 安装

```bash
cd sdk/typescript
npm install
npm run build
```

### 使用示例

```typescript
import { MemoryClient, MemoryPool, MemoryType } from 'self-evolving-memory'

const client = new MemoryClient('http://localhost:3000')

// 创建记忆
const memory = await client.create({
  content: '用户喜欢简洁回复',
  pool: MemoryPool.Implicit,
  type: MemoryType.Preference,
  tags: ['communication']
})

// 搜索记忆
const results = await client.search({ query: '用户偏好', limit: 5 })

// 获取统计
const stats = await client.stats()

// 创建链接
await client.link(sourceId, targetId, 'related')
```

---

## 🤖 MCP 工具

### 工具列表

| 工具名 | 说明 |
|--------|------|
| memory_create | 创建记忆 |
| memory_retrieve | 搜索记忆 |
| memory_get | 获取单个记忆 |
| memory_update | 更新记忆 |
| memory_delete | 删除记忆 |
| memory_link | 创建链接 |
| memory_stats | 获取统计 |
| memory_consolidate | 整合记忆 |

### 使用示例 (Claude)

```json
{
  "tool": "memory_create",
  "arguments": {
    "content": "用户偏好使用中文交流",
    "pool": "implicit",
    "type": "preference"
  }
}
```

---

## 🔧 高级用法

### 渐进式检索

```rust
use self_evolving_memory::memory::{InMemoryStore, SpreadingActivation};
use std::sync::Arc;

let store = Arc::new(InMemoryStore::new());
let spreader = SpreadingActivation::new(store.clone());

// 从种子记忆开始扩散
let results = spreader.spread(&[seed_id]).await?;

// 渐进式搜索
let results = spreader.progressive_search("用户偏好", 5).await?;
```

### Memory Consolidation

```rust
use self_evolving_memory::memory::{InMemoryStore, MemoryConsolidator};
use std::sync::Arc;

let store = Arc::new(InMemoryStore::new());
let consolidator = MemoryConsolidator::new(store.clone());

// 运行整合
let report = consolidator.consolidate().await?;
println!("Decayed: {}, Removed: {}", report.decayed_count, report.removed_count);
```

### Embedding 服务

```rust
use self_evolving_memory::memory::embedding::EmbeddingService;

// 使用 OpenAI
let service = EmbeddingService::openai("sk-xxx".to_string());
let embedding = service.embed("这是一段文本").await?;

// 使用本地模拟
let service = EmbeddingService::local(1536);
let embedding = service.embed("这是一段文本").await?;
```

---

## 📊 数据模型

### Memory

| 字段 | 类型 | 说明 |
|------|------|------|
| id | UUID | 唯一标识 |
| content | String | 记忆内容 |
| pool | Enum | explicit/implicit |
| type | Enum | 6种类型 |
| confidence | f64 | 置信度 (0-1) |
| importance | f64 | 重要度 (0-1) |
| decay_rate | f64 | 衰减率 |
| tags | Vec\<String\> | 标签 |
| created_at | DateTime | 创建时间 |
| access_count | u64 | 访问次数 |

### MemoryType

- `fact` - 事实知识
- `event` - 事件经历
- `procedure` - 方法技能
- `concept` - 概念理解
- `preference` - 偏好习惯
- `context` - 当前状态

### LinkType

- `related` - 相关
- `causes` - 因果
- `contradicts` - 矛盾
- `specializes` - 特化
- `derived_from` - 来源
- `similar` - 相似
- `follows` - 顺序
- `alternative` - 替代

---

## 🎯 最佳实践

### 1. 记忆分类

```python
# 明确的事实 → Explicit/Fact
client.create({"content": "公司API地址是api.example.com", "pool": "explicit", "type": "fact"})

# 用户偏好 → Implicit/Preference
client.create({"content": "用户喜欢简洁回复", "pool": "implicit", "type": "preference"})

# 当前状态 → Explicit/Context
client.create({"content": "当前在讨论项目需求", "pool": "explicit", "type": "context"})
```

### 2. 链接建立

```python
# 建立因果链
client.link(cause_id, effect_id, "causes")

# 建立相关链
client.link(related1_id, related2_id, "related")

# 建立相似链
client.link(similar1_id, similar2_id, "similar")
```

### 3. 定期整理

```python
# 每天运行一次 consolidation
consolidator.consolidate()

# 检查统计
stats = client.stats()
if stats['total_memories'] > 1000:
    print("考虑清理旧记忆")
```

---

## ❓ 常见问题

### Q: 如何选择 pool?
- **Explicit**: 用户明确告知的信息
- **Implicit**: 系统观察推断的信息

### Q: 如何设置 importance?
- 0.0-0.3: 低重要度
- 0.4-0.7: 中等重要度
- 0.8-1.0: 高重要度

### Q: 记忆会自动删除吗?
- 设置 `decay_rate` 后，记忆强度会随时间衰减
- 当强度低于阈值时，`consolidate()` 会自动清理

---

## 📝 更新日志

### v0.1.0 (2026-04-04)
- ✅ Phase 1: 核心功能
- ✅ Phase 2: 高级特性
- ✅ 渐进式检索
- ✅ Memory Consolidation
- ✅ Embedding 支持