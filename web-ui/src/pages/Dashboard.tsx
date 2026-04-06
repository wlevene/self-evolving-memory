import { useQuery } from '@tanstack/react-query'
import { api } from '../api/client'
import { formatDistanceToNow } from 'date-fns'
import { 
  Brain, 
  Link2, 
  TrendingUp, 
  Activity,
  Database,
  Zap,
  Target
} from 'lucide-react'

interface Stats {
  total_memories: number
  explicit_count: number
  implicit_count: number
  total_links: number
  by_type: Record<string, number>
  by_tag: Record<string, number>
  avg_confidence: number
  avg_importance: number
}

export function Dashboard() {
  const { data: stats, isLoading } = useQuery<Stats>({
    queryKey: ['stats'],
    queryFn: async () => {
      const response = await api.get('/stats')
      return response.data
    },
  })

  const { data: recent } = useQuery({
    queryKey: ['recent'],
    queryFn: async () => {
      const response = await api.get('/memories?limit=5')
      return response.data.results
    },
  })

  if (isLoading) {
    return (
      <div className="flex items-center justify-center py-12">
        <div className="animate-spin rounded-full h-8 w-8 border-b-2 border-indigo-600"></div>
      </div>
    )
  }

  return (
    <div className="space-y-8">
      {/* Stats Grid */}
      <div className="grid grid-cols-1 md:grid-cols-2 lg:grid-cols-4 gap-6">
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-500">Total Memories</p>
              <p className="text-2xl font-bold text-gray-900">{stats?.total_memories || 0}</p>
            </div>
            <Brain className="w-8 h-8 text-indigo-600" />
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-500">Explicit Pool</p>
              <p className="text-2xl font-bold text-gray-900">{stats?.explicit_count || 0}</p>
            </div>
            <Database className="w-8 h-8 text-green-600" />
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-500">Implicit Pool</p>
              <p className="text-2xl font-bold text-gray-900">{stats?.implicit_count || 0}</p>
            </div>
            <Zap className="w-8 h-8 text-purple-600" />
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between">
            <div>
              <p className="text-sm text-gray-500">Total Links</p>
              <p className="text-2xl font-bold text-gray-900">{stats?.total_links || 0}</p>
            </div>
            <Link2 className="w-8 h-8 text-blue-600" />
          </div>
        </div>
      </div>

      {/* Confidence & Importance */}
      <div className="grid grid-cols-1 md:grid-cols-2 gap-6">
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center gap-2 mb-4">
            <Target className="w-5 h-5 text-indigo-600" />
            <h3 className="text-lg font-semibold">Average Confidence</h3>
          </div>
          <div className="flex items-center gap-4">
            <div className="flex-1 h-2 bg-gray-200 rounded-full overflow-hidden">
              <div 
                className="h-full bg-indigo-600 rounded-full" 
                style={{ width: `${(stats?.avg_confidence || 0) * 100}%` }}
              ></div>
            </div>
            <span className="text-lg font-bold text-gray-900">
              {(stats?.avg_confidence || 0).toFixed(2)}
            </span>
          </div>
        </div>

        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center gap-2 mb-4">
            <TrendingUp className="w-5 h-5 text-green-600" />
            <h3 className="text-lg font-semibold">Average Importance</h3>
          </div>
          <div className="flex items-center gap-4">
            <div className="flex-1 h-2 bg-gray-200 rounded-full overflow-hidden">
              <div 
                className="h-full bg-green-600 rounded-full" 
                style={{ width: `${(stats?.avg_importance || 0) * 100}%` }}
              ></div>
            </div>
            <span className="text-lg font-bold text-gray-900">
              {(stats?.avg_importance || 0).toFixed(2)}
            </span>
          </div>
        </div>
      </div>

      {/* By Type Chart */}
      {stats?.by_type && Object.keys(stats.by_type).length > 0 && (
        <div className="bg-white rounded-lg shadow p-6">
          <h3 className="text-lg font-semibold mb-4">Memories by Type</h3>
          <div className="grid grid-cols-2 md:grid-cols-3 lg:grid-cols-6 gap-4">
            {Object.entries(stats.by_type).map(([type, count]) => (
              <div key={type} className="text-center p-4 bg-gray-50 rounded-lg">
                <p className="text-sm text-gray-500 capitalize">{type}</p>
                <p className="text-xl font-bold text-gray-900">{count}</p>
              </div>
            ))}
          </div>
        </div>
      )}

      {/* Recent Memories */}
      {recent && recent.length > 0 && (
        <div className="bg-white rounded-lg shadow p-6">
          <div className="flex items-center justify-between mb-4">
            <h3 className="text-lg font-semibold">Recent Memories</h3>
            <Activity className="w-5 h-5 text-gray-400" />
          </div>
          <div className="space-y-4">
            {recent.map((memory: any) => (
              <div key={memory.id} className="flex items-start gap-4 p-4 bg-gray-50 rounded-lg">
                <div className="flex-1">
                  <p className="text-sm text-gray-900 truncate">{memory.content}</p>
                  <div className="flex items-center gap-2 mt-2 text-xs text-gray-500">
                    <span className="px-2 py-1 bg-indigo-100 text-indigo-700 rounded">
                      {memory.pool}
                    </span>
                    <span className="px-2 py-1 bg-gray-100 text-gray-700 rounded">
                      {memory.type}
                    </span>
                    <span>
                      {formatDistanceToNow(new Date(memory.created_at), { addSuffix: true })}
                    </span>
                  </div>
                </div>
              </div>
            ))}
          </div>
        </div>
      )}
    </div>
  )
}