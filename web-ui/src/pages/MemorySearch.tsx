import { useState } from 'react'
import { useQuery } from '@tanstack/react-query'
import { Link } from 'react-router-dom'
import { api } from '../api/client'
import { Search, Brain, Eye, Filter, X } from 'lucide-react'

export function MemorySearch() {
  const [query, setQuery] = useState('')
  const [pool, setPool] = useState('')
  const [type, setType] = useState('')
  const [minConfidence, setMinConfidence] = useState(0.5)
  const [limit, setLimit] = useState(10)
  const [searching, setSearching] = useState(false)

  const { data, isLoading, refetch } = useQuery({
    queryKey: ['search', query, pool, type, minConfidence, limit],
    queryFn: async () => {
      const params = new URLSearchParams()
      params.set('query', query)
      params.set('min_confidence', String(minConfidence))
      params.set('limit', String(limit))
      if (pool) params.set('pool', pool)
      if (type) params.set('type', type)
      
      const response = await api.get(`/memories/search?${params}`)
      return response.data
    },
    enabled: searching && query.length > 0,
  })

  const handleSearch = () => {
    if (query.length > 0) {
      setSearching(true)
      refetch()
    }
  }

  const handleClear = () => {
    setQuery('')
    setPool('')
    setType('')
    setMinConfidence(0.5)
    setLimit(10)
    setSearching(false)
  }

  return (
    <div className="space-y-6">
      {/* Search Form */}
      <div className="bg-white rounded-lg shadow p-6">
        <div className="flex items-center gap-2 mb-4">
          <Search className="w-5 h-5 text-indigo-600" />
          <h2 className="text-lg font-semibold">Search Memories</h2>
        </div>

        {/* Query input */}
        <div className="mb-4">
          <label className="block text-sm text-gray-600 mb-1">Query</label>
          <input
            type="text"
            value={query}
            onChange={(e) => setQuery(e.target.value)}
            onKeyDown={(e) => e.key === 'Enter' && handleSearch()}
            placeholder="Search memory content..."
            className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
          />
        </div>

        {/* Filters */}
        <div className="grid grid-cols-2 md:grid-cols-4 gap-4 mb-4">
          <div>
            <label className="block text-sm text-gray-600 mb-1">Pool</label>
            <select
              value={pool}
              onChange={(e) => setPool(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
            >
              <option value="">All</option>
              <option value="explicit">Explicit</option>
              <option value="implicit">Implicit</option>
            </select>
          </div>
          <div>
            <label className="block text-sm text-gray-600 mb-1">Type</label>
            <select
              value={type}
              onChange={(e) => setType(e.target.value)}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
            >
              <option value="">All</option>
              <option value="fact">Fact</option>
              <option value="event">Event</option>
              <option value="procedure">Procedure</option>
              <option value="concept">Concept</option>
              <option value="preference">Preference</option>
              <option value="context">Context</option>
            </select>
          </div>
          <div>
            <label className="block text-sm text-gray-600 mb-1">Min Confidence</label>
            <input
              type="range"
              min="0"
              max="1"
              step="0.1"
              value={minConfidence}
              onChange={(e) => setMinConfidence(Number(e.target.value))}
              className="w-full"
            />
            <div className="text-xs text-gray-500 text-center">
              {(minConfidence * 100).toFixed(0)}%
            </div>
          </div>
          <div>
            <label className="block text-sm text-gray-600 mb-1">Limit</label>
            <select
              value={limit}
              onChange={(e) => setLimit(Number(e.target.value))}
              className="w-full px-3 py-2 border border-gray-300 rounded-md focus:outline-none focus:ring-2 focus:ring-indigo-500"
            >
              <option value="5">5</option>
              <option value="10">10</option>
              <option value="20">20</option>
              <option value="50">50</option>
            </select>
          </div>
        </div>

        {/* Actions */}
        <div className="flex items-center gap-3">
          <button
            onClick={handleSearch}
            disabled={query.length === 0}
            className="px-4 py-2 bg-indigo-600 text-white rounded-md hover:bg-indigo-700 disabled:bg-gray-300 disabled:cursor-not-allowed transition-colors"
          >
            Search
          </button>
          <button
            onClick={handleClear}
            className="px-4 py-2 text-gray-600 hover:text-gray-900 transition-colors flex items-center gap-1"
          >
            <X className="w-4 h-4" />
            Clear
          </button>
        </div>
      </div>

      {/* Results */}
      {isLoading && (
        <div className="flex items-center justify-center py-12">
          <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600"></div>
        </div>
      )}

      {!isLoading && searching && data?.results && (
        <div className="space-y-4">
          <div className="flex items-center justify-between">
            <h3 className="text-lg font-semibold text-gray-900">
              Results ({data.count})
            </h3>
            {data.query && (
              <span className="text-sm text-gray-500">
                Query: "{data.query}"
              </span>
            )}
          </div>

          {data.results.length > 0 ? (
            <div className="bg-white rounded-lg shadow overflow-hidden">
              <div className="divide-y divide-gray-200">
                {data.results.map((memory: any) => (
                  <div key={memory.id} className="p-4 hover:bg-gray-50">
                    <div className="flex items-start justify-between gap-4">
                      <div className="flex-1">
                        <p className="text-sm text-gray-900">{memory.content}</p>
                        <div className="flex items-center gap-2 mt-2">
                          <span className={`px-2 py-1 text-xs rounded ${
                            memory.pool === 'explicit'
                              ? 'bg-green-100 text-green-800'
                              : 'bg-purple-100 text-purple-800'
                          }`}>
                            {memory.pool}
                          </span>
                          <span className="px-2 py-1 text-xs rounded bg-gray-100 text-gray-800">
                            {memory.type}
                          </span>
                          <span className="text-xs text-gray-500">
                            {(memory.confidence * 100).toFixed(0)}% confidence
                          </span>
                        </div>
                      </div>
                      <Link
                        to={`/memories/${memory.id}`}
                        className="text-indigo-600 hover:text-indigo-900 p-2"
                      >
                        <Eye className="w-4 h-4" />
                      </Link>
                    </div>
                  </div>
                ))}
              </div>
            </div>
          ) : (
            <div className="bg-white rounded-lg shadow p-8 text-center">
              <Brain className="w-12 h-12 text-gray-400 mx-auto mb-4" />
              <p className="text-gray-500">No memories found matching your query</p>
            </div>
          )}
        </div>
      )}

      {!searching && (
        <div className="bg-white rounded-lg shadow p-8 text-center">
          <Filter className="w-12 h-12 text-gray-400 mx-auto mb-4" />
          <p className="text-gray-500">
            Enter a search query and click Search to find memories
          </p>
        </div>
      )}
    </div>
  )
}