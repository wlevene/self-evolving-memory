# Self-Evolving Memory

> **AI Agent Memory System** - Production-grade memory infrastructure

🧠 一个为 AI Agent 设计的记忆系统，实现类似人类的记忆管理机制。

## 特性

- **双池机制**: Explicit (显性) + Implicit (隐性) 记忆
- **六种类型**: Fact, Event, Procedure, Concept, Preference, Context
- **记忆链接**: 8种关系类型 (related, causes, similar...)
- **渐进式检索**: Spreading Activation 算法
- **记忆衰减**: 模拟人类遗忘曲线
- **MCP 协议**: 8个工具供 AI Agent 调用
- **多语言 SDK**: Python + TypeScript
- **Web UI**: React 管理界面
- **CLI**: 完整命令行工具

## 快速开始

```bash
# 构建
cargo build --release

# 启动服务器
./target/debug/mem --serve

# 创建记忆
./target/debug/mem create "用户喜欢简洁回复" --pool implicit --type preference

# 搜索
./target/debug/mem search "用户偏好"

# 交互模式
./target/debug/mem interactive
```

## 文档

- [使用文档](docs/USAGE.md)
- [API 文档](docs/API.md)
- [架构设计](docs/ARCHITECTURE.md)

## 架构

```
┌─────────────────────────────────────────┐
│          Self-Evolving Memory           │
├─────────────────────────────────────────┤
│  ┌─────────┐  ┌─────────┐  ┌─────────┐  │
│  │ CLI     │  │ HTTP API│  │ MCP     │  │
│  └────┬────┘  └────┬────┘  └────┬────┘  │
│       │            │            │        │
│       └────────────┼────────────┘        │
│                    │                     │
│  ┌─────────────────▼─────────────────┐  │
│  │         Memory Core               │  │
│  │  • InMemoryStore                  │  │
│  │  • SpreadingActivation            │  │
│  │  • MemoryConsolidator             │  │
│  │  • EmbeddingService               │  │
│  └───────────────────────────────────┘  │
│                                         │
└─────────────────────────────────────────┘
```

## 核心概念

### 双池机制

**Explicit Pool (显性记忆)**
- 有意识回忆
- 事实、事件、方法
- 用户明确告知的信息

**Implicit Pool (隐性记忆)**
- 无意识影响
- 偏好、习惯、模式
- 系统观察推断的信息

### 渐进式检索

```
搜索"用户偏好"
    ↓
找到 3 条直接匹配
    ↓
顺着链接扩散
    ↓
找到 8 条相关记忆
```

## SDK 使用

### Python

```python
from self_evolving_memory import MemoryClient

client = MemoryClient("http://localhost:3000")

# 创建
memory = client.create({
    "content": "用户偏好中文交流",
    "pool": "implicit",
    "type": "preference"
})

# 搜索
results = client.search("用户偏好")
```

### TypeScript

```typescript
import { MemoryClient } from 'self-evolving-memory'

const client = new MemoryClient('http://localhost:3000')

const memory = await client.create({
  content: '用户偏好中文交流',
  pool: 'implicit',
  type: 'preference'
})
```

## 项目结构

```
self-evolving-memory/
├── src/
│   ├── main.rs           # CLI 入口
│   ├── memory/           # 核心模块
│   │   ├── types.rs      # 数据类型
│   │   ├── store.rs      # 存储
│   │   ├── spreading.rs  # 渐进式检索
│   │   ├── consolidation.rs
│   │   └── embedding.rs
│   ├── api/              # HTTP API
│   └── mcp/              # MCP Server
├── sdk/
│   ├── python/           # Python SDK
│   └── typescript/       # TypeScript SDK
├── web-ui/               # React UI
└── docs/                 # 文档
```

## License

MIT