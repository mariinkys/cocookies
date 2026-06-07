import api from '@/api/axios'
import type { RegisterPayload } from '@/types/auth.types'
import type { PageResponse, PaginatedParams } from '@/types/common.types'
import type { UpdatePasswordPayload, UserResponse, UserUpdatePayload } from '@/types/user.types'

class UsersService {
  async getAll(params?: PaginatedParams): Promise<PageResponse<UserResponse>> {
    const { data } = await api.get<PageResponse<UserResponse>>('/api/users', { params })
    return data
  }

  async getById(id: string): Promise<UserResponse> {
    const { data } = await api.get<UserResponse>(`/api/users/${id}`)
    return data
  }

  async create(payload: RegisterPayload): Promise<UserResponse> {
    const { data } = await api.post<UserResponse>('/api/auth/register', payload)
    return data
  }

  async update(id: string, payload: UserUpdatePayload): Promise<UserResponse> {
    const { data } = await api.put<UserResponse>(`/api/users/${id}`, payload)
    return data
  }

  async updatePassword(id: string, payload: UpdatePasswordPayload): Promise<void> {
    const { data } = await api.patch<void>(`/api/users/${id}/password`, payload)
    return data
  }

  async delete(id: string): Promise<void> {
    const { data } = await api.delete<void>(`/api/users/${id}`)
    return data
  }
}

export const usersService = new UsersService()
