import api from '@/api/axios'

class SystemService {
  async getServerVersion(): Promise<string> {
    const { data } = await api.get<string>('/api/system/version')
    return data
  }
}

export const systemService = new SystemService()
