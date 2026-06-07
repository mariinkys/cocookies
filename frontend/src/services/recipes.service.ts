import api from '@/api/axios'
import type { PageResponse, PaginatedParams } from '@/types/common.types'
import type { RecipeRequest, RecipeResponse, RecipeTemplate } from '@/types/recipe.types'

class RecipesService {
  async getAll(params?: PaginatedParams): Promise<PageResponse<RecipeResponse>> {
    const { data } = await api.get<PageResponse<RecipeResponse>>('/api/recipes', { params })
    return data
  }

  async getAllShared(params?: PaginatedParams): Promise<PageResponse<RecipeResponse>> {
    const { data } = await api.get<PageResponse<RecipeResponse>>('/api/recipes/shared', { params })
    return data
  }

  async getById(id: string): Promise<RecipeResponse> {
    const { data } = await api.get<RecipeResponse>(`/api/recipes/${id}`)
    return data
  }

  async create(payload: RecipeRequest): Promise<RecipeResponse> {
    const { data } = await api.post<RecipeResponse>('/api/recipes', payload)
    return data
  }

  async update(id: string, payload: RecipeRequest): Promise<RecipeResponse> {
    const { data } = await api.put<RecipeResponse>(`/api/recipes/${id}`, payload)
    return data
  }

  async delete(id: string): Promise<void> {
    const { data } = await api.delete<void>(`/api/recipes/${id}`)
    return data
  }

  async downloadPdf(id: string, template: RecipeTemplate): Promise<void> {
    const response = await api.get(`/api/recipes/${id}/pdf/${template}`, {
      responseType: 'blob',
    })
    const url = URL.createObjectURL(response.data)
    const a = document.createElement('a')
    a.href = url
    a.download = `recipe-${id}.pdf`
    a.click()
    URL.revokeObjectURL(url)
  }
}

export const recipesService = new RecipesService()
