import api from '@/api/axios'
import type { PublicConfig } from '@/types/system.types'

class SystemService {
  async getServerVersion(): Promise<string> {
    const { data } = await api.get<string>('/api/system/version')
    return data
  }

  async getConfig(): Promise<PublicConfig> {
    const { data } = await api.get<PublicConfig>('/api/system/config')
    return data
  }
}

export const systemService = new SystemService()
