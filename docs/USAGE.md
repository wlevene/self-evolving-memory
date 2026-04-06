# Self-Evolving Memory Usage Documentation

## 🚀 Quick Start

### Installation

```bash
cd ~/workspace/self-evolving-memory
cargo build --release
```

### Start the Server

By default, the server runs with an **In-Memory** store (data will be lost upon restart). You can also configure it to use SQLite or PostgreSQL for persistence.

```bash
# Method 1: Using CLI (In-Memory)
./target/debug/mem --serve --port 3000

# Method 2: Using cargo (In-Memory)
cargo run -- --serve --port 3000
```

#### Use SQLite for Local Persistence
If you want to save data locally without setting up a database server, set the `DATABASE_URL` to a SQLite file path:

```bash
export DATABASE_URL="sqlite://memory.db?mode=rwc"
cargo run -- --serve --port 3000
```

#### Use PostgreSQL for Production
For production deployments and pgvector support, set `DATABASE_URL` to a PostgreSQL connection string:

```bash
export DATABASE_URL="postgres://postgres:postgres@localhost:5432/memory"
cargo run -- --serve --port 3000
```

The server will start at `http://localhost:3000`.

---

## 📖 CLI Usage

### Basic Commands

```bash
# View help
mem --help

# Create memory
mem create "User likes concise replies" --pool implicit --type preference

# Search memory
mem search "User preference" --limit 10

# List all memories
mem list --pool explicit --limit 20

# Get a single memory
mem get <uuid>

# Update memory
mem update <uuid> --importance 0.9

# Delete memory
mem delete <uuid>

# View statistics
mem stats

# Interactive mode (REPL)
mem interactive
```

### Interactive Mode

```bash
$ mem interactive
mem> help
mem> create This is a test memory
mem> search test
mem> list
mem> stats
mem> exit
```

---

## 🌐 HTTP API

### Base Endpoints

| Method | Endpoint | Description |
|------|------|------|
| GET | /health | Health check |
| GET | /stats | System statistics |

### Memory Operations

#### Create Memory
```bash
curl -X POST http://localhost:3000/memories \
  -H "Content-Type: application/json" \
  -d '{
    "content": "User likes concise replies",
    "pool": "implicit",
    "type": "preference",
    "importance": 0.8,
    "tags": ["user", "communication"]
  }'
```

#### Get Memory
```bash
curl http://localhost:3000/memories/<uuid>
```

#### Update Memory
```bash
curl -X PUT http://localhost:3000/memories/<uuid> \
  -H "Content-Type: application/json" \
  -d '{"importance": 0.9}'
```

#### Delete Memory
```bash
curl -X DELETE http://localhost:3000/memories/<uuid>
```

#### List Memories
```bash
curl "http://localhost:3000/memories?pool=explicit&limit=10"
```

#### Search Memories
```bash
curl "http://localhost:3000/memories/search?query=User preference&limit=5"
```

### Link Operations

#### Create Link
```bash
curl -X POST http://localhost:3000/links \
  -H "Content-Type: application/json" \
  -d '{
    "source_id": "<uuid1>",
    "target_id": "<uuid2>",
    "link_type": "related"
  }'
```

#### Get Memory Links
```bash
curl http://localhost:3000/memories/<uuid>/links
```

---

## 🐍 Python SDK

### Installation

```bash
cd sdk/python
pip install -e .
```

### Usage Example

```python
from self_evolving_memory import MemoryClient, MemoryPool, MemoryType

# Create client
client = MemoryClient("http://localhost:3000")

# Create memory
memory = client.create({
    "content": "User likes concise replies",
    "pool": "implicit",
    "type": "preference",
    "tags": ["communication"]
})
print(f"Created: {memory['id']}")

# Search memory
results = client.search("User preference", limit=5)
for m in results:
    print(f"- {m['content']}")

# Get statistics
stats = client.stats()
print(f"Total memories: {stats['total_memories']}")

# Create link
client.link(source_id, target_id, "related")

# Progressive retrieval (requires SpreadingActivation)
# See Advanced Usage section
```

---

## 📦 TypeScript SDK

### Installation

```bash
cd sdk/typescript
npm install
npm run build
```

### Usage Example

```typescript
import { MemoryClient, MemoryPool, MemoryType } from 'self-evolving-memory'

const client = new MemoryClient('http://localhost:3000')

// Create memory
const memory = await client.create({
  content: 'User likes concise replies',
  pool: MemoryPool.Implicit,
  type: MemoryType.Preference,
  tags: ['communication']
})

// Search memory
const results = await client.search({ query: 'User preference', limit: 5 })

// Get statistics
const stats = await client.stats()

// Create link
await client.link(sourceId, targetId, 'related')
```

---

## 🤖 MCP Tools

### Tool List

| Tool Name | Description |
|--------|------|
| memory_create | Create memory |
| memory_retrieve | Search memory |
| memory_get | Get a single memory |
| memory_update | Update memory |
| memory_delete | Delete memory |
| memory_link | Create link |
| memory_stats | Get statistics |
| memory_consolidate | Consolidate memory |

### Usage Example (Claude)

```json
{
  "tool": "memory_create",
  "arguments": {
    "content": "User prefers communicating in Chinese",
    "pool": "implicit",
    "type": "preference"
  }
}
```

---

## 🔧 Advanced Usage

### Progressive Retrieval

```rust
use self_evolving_memory::memory::{InMemoryStore, SpreadingActivation};
use std::sync::Arc;

let store = Arc::new(InMemoryStore::new());
let spreader = SpreadingActivation::new(store.clone());

// Spread from seed memories
let results = spreader.spread(&[seed_id]).await?;

// Progressive search
let results = spreader.progressive_search("User preference", 5).await?;
```

### Memory Consolidation

```rust
use self_evolving_memory::memory::{InMemoryStore, MemoryConsolidator};
use std::sync::Arc;

let store = Arc::new(InMemoryStore::new());
let consolidator = MemoryConsolidator::new(store.clone());

// Run consolidation
let report = consolidator.consolidate().await?;
println!("Decayed: {}, Removed: {}", report.decayed_count, report.removed_count);
```

### Embedding Service

```rust
use self_evolving_memory::memory::embedding::EmbeddingService;

// Use OpenAI
let service = EmbeddingService::openai("sk-xxx".to_string());
let embedding = service.embed("This is a text snippet").await?;

// Use local mock
let service = EmbeddingService::local(1536);
let embedding = service.embed("This is a text snippet").await?;
```

---

## 📊 Data Model

### Memory

| Field | Type | Description |
|------|------|------|
| id | UUID | Unique identifier |
| content | String | Memory content |
| pool | Enum | explicit/implicit |
| type | Enum | 6 types |
| confidence | f64 | Confidence (0-1) |
| importance | f64 | Importance (0-1) |
| decay_rate | f64 | Decay rate |
| tags | Vec\<String\> | Tags |
| created_at | DateTime | Creation time |
| access_count | u64 | Access count |

### MemoryType

- `fact` - Factual knowledge
- `event` - Event experience
- `procedure` - Procedure/Skill
- `concept` - Concept understanding
- `preference` - Preference/Habit
- `context` - Current state/Context

### LinkType

- `related` - Related
- `causes` - Causes
- `contradicts` - Contradicts
- `specializes` - Specializes
- `derived_from` - Derived from
- `similar` - Similar
- `follows` - Follows
- `alternative` - Alternative

---

## 🎯 Best Practices

### 1. Memory Classification

```python
# Explicit facts -> Explicit/Fact
client.create({"content": "Company API address is api.example.com", "pool": "explicit", "type": "fact"})

# User preferences -> Implicit/Preference
client.create({"content": "User likes concise replies", "pool": "implicit", "type": "preference"})

# Current context -> Explicit/Context
client.create({"content": "Currently discussing project requirements", "pool": "explicit", "type": "context"})
```

### 2. Link Establishment

```python
# Establish causal chain
client.link(cause_id, effect_id, "causes")

# Establish related chain
client.link(related1_id, related2_id, "related")

# Establish similar chain
client.link(similar1_id, similar2_id, "similar")
```

### 3. Regular Consolidation

```python
# Run consolidation once a day
consolidator.consolidate()

# Check statistics
stats = client.stats()
if stats['total_memories'] > 1000:
    print("Consider cleaning up old memories")
```

---

## ❓ FAQ

### Q: How to choose a pool?
- **Explicit**: Information explicitly told by the user
- **Implicit**: Information inferred from system observation

### Q: How to set importance?
- 0.0-0.3: Low importance
- 0.4-0.7: Medium importance
- 0.8-1.0: High importance

### Q: Will memories be deleted automatically?
- After setting `decay_rate`, memory strength decays over time
- When strength falls below the threshold, `consolidate()` cleans it up automatically

---

## 📝 Changelog

### v0.1.0 (2026-04-04)
- ✅ Phase 1: Core features
- ✅ Phase 2: Advanced features
- ✅ Progressive Retrieval
- ✅ Memory Consolidation
- ✅ Embedding support
