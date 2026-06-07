export interface CategoryRequest {
  name: string
  description: string | null
}

export interface CategoryResponse {
  id: string
  userId: string
  name: string
  description: string | null
  createdAt: string
  updatedAt: string
}
