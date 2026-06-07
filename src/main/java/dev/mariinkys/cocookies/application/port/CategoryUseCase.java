package dev.mariinkys.cocookies.application.port;

import dev.mariinkys.cocookies.application.service.RequesterContext;
import dev.mariinkys.cocookies.domain.models.Category;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;

import java.util.List;
import java.util.UUID;

public interface CategoryUseCase {
    Category createCategory(String name, String description, RequesterContext requester);
    Category getCategoryById(UUID id, RequesterContext requester);
    Page<Category> getAllCategories(String search, Pageable pageable, RequesterContext requester);
    List<Category> getSelector(RequesterContext requester);
    Category updateCategory(UUID id, String name, String description, RequesterContext requester);
    void deleteCategory(UUID id, RequesterContext requester);
}