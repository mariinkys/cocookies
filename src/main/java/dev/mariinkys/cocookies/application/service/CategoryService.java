package dev.mariinkys.cocookies.application.service;

import dev.mariinkys.cocookies.application.exception.CategoryNotFoundException;
import dev.mariinkys.cocookies.application.port.CategoryUseCase;
import dev.mariinkys.cocookies.domain.models.Category;
import dev.mariinkys.cocookies.domain.repository.CategoryRepository;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.security.access.AccessDeniedException;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.List;
import java.util.UUID;

@Service
public class CategoryService implements CategoryUseCase {

    private final CategoryRepository categoryRepository;

    public CategoryService(CategoryRepository categoryRepository) {
        this.categoryRepository = categoryRepository;
    }

    @Override
    @Transactional
    public Category createCategory(String name, String description, RequesterContext requester) {
        return categoryRepository.save(new Category(requester.id(), name, description));
    }

    @Override
    @Transactional(readOnly = true)
    public Category getCategoryById(UUID id,  RequesterContext requester) {
        Category target = categoryRepository.findById(id)
                .orElseThrow(() -> new CategoryNotFoundException(id));

        if (!target.getId().equals(requester.id())) {
            throw new AccessDeniedException("You can only access your own category");
        }

        return target;
    }

    @Override
    @Transactional(readOnly = true)
    public Page<Category> getAllCategories(String search, Pageable pageable, RequesterContext requester) {
        return categoryRepository.findAllByUserId(requester.id(), search, pageable);
    }

    @Override
    @Transactional(readOnly = true)
    public List<Category> getSelector(RequesterContext requester) {
        return categoryRepository.getSelector(requester.id());
    }

    @Override
    @Transactional
    public Category updateCategory(UUID id, String name, String description, RequesterContext requester) {
        Category existing = getCategoryById(id, requester);

        if (!existing.getName().equals(name) &&
                categoryRepository.existsByUserIdAndName(existing.getUserId(), name))
            throw new IllegalArgumentException("Category with name '" + name + "' already exists");

        return categoryRepository.save(existing.withUpdatedDetails(name, description));
    }

    @Override
    @Transactional
    public void deleteCategory(UUID id, RequesterContext requester) {
        getCategoryById(id, requester);
        categoryRepository.deleteById(id);
    }
}