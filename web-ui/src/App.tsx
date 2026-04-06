import { BrowserRouter as Router, Routes, Route, Link } from 'react-router-dom'
import { QueryClient, QueryClientProvider } from '@tanstack/react-query'
import { MemoryList } from './pages/MemoryList'
import { MemoryDetail } from './pages/MemoryDetail'
import { MemorySearch } from './pages/MemorySearch'
import { Dashboard } from './pages/Dashboard'
import { NewMemory } from './pages/NewMemory'
import { Brain, Activity, Search, Plus } from 'lucide-react'

const queryClient = new QueryClient()

function App() {
  return (
    <QueryClientProvider client={queryClient}>
      <Router>
        <div className="min-h-screen bg-gray-50">
          {/* Header */}
          <header className="bg-white shadow-sm border-b border-gray-200">
            <div className="max-w-7xl mx-auto px-4 py-4">
              <div className="flex items-center justify-between">
                <div className="flex items-center gap-2">
                  <Brain className="w-8 h-8 text-indigo-600" />
                  <h1 className="text-xl font-bold text-gray-900">
                    Self-Evolving Memory
                  </h1>
                </div>
                <nav className="flex items-center gap-4">
                  <Link
                    to="/"
                    className="flex items-center gap-1 px-3 py-2 text-sm text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded-md transition-colors"
                  >
                    <Activity className="w-4 h-4" />
                    Dashboard
                  </Link>
                  <Link
                    to="/memories"
                    className="flex items-center gap-1 px-3 py-2 text-sm text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded-md transition-colors"
                  >
                    <Brain className="w-4 h-4" />
                    Memories
                  </Link>
                  <Link
                    to="/search"
                    className="flex items-center gap-1 px-3 py-2 text-sm text-gray-600 hover:text-gray-900 hover:bg-gray-100 rounded-md transition-colors"
                  >
                    <Search className="w-4 h-4" />
                    Search
                  </Link>
                  <Link
                    to="/new"
                    className="flex items-center gap-1 px-3 py-2 text-sm bg-indigo-600 text-white hover:bg-indigo-700 rounded-md transition-colors"
                  >
                    <Plus className="w-4 h-4" />
                    New Memory
                  </Link>
                </nav>
              </div>
            </div>
          </header>

          {/* Main content */}
          <main className="max-w-7xl mx-auto px-4 py-8">
            <Routes>
              <Route path="/" element={<Dashboard />} />
              <Route path="/memories" element={<MemoryList />} />
              <Route path="/memories/:id" element={<MemoryDetail />} />
              <Route path="/search" element={<MemorySearch />} />
              <Route path="/new" element={<NewMemory />} />
            </Routes>
          </main>
        </div>
      </Router>
    </QueryClientProvider>
  )
}

export default App