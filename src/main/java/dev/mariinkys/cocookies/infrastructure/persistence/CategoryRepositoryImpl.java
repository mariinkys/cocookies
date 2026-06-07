package dev.mariinkys.cocookies.infrastructure.persistence;

import dev.mariinkys.cocookies.domain.models.Category;
import dev.mariinkys.cocookies.domain.repository.CategoryRepository;
import dev.mariinkys.cocookies.infrastructure.persistence.mapper.CategoryMapper;
import dev.mariinkys.cocookies.infrastructure.persistence.repository.CategoryJpaRepository;
import dev.mariinkys.cocookies.infrastructure.utils.PaginationUtils;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.PageImpl;
import org.springframework.data.domain.Pageable;
import org.springframework.stereotype.Component;

import java.util.List;
import java.util.Optional;
import java.util.UUID;

@Component
public class CategoryRepositoryImpl implements CategoryRepository {

    private final CategoryJpaRepository jpaRepository;
    private final CategoryMapper mapper;

    public CategoryRepositoryImpl(CategoryJpaRepository jpaRepository, CategoryMapper mapper) {
        this.jpaRepository = jpaRepository;
        this.mapper = mapper;
    }

    @Override
    public Category save(Category category) {
        return mapper.toDomain(jpaRepository.save(mapper.toEntity(category)));
    }

    @Override
    public Optional<Category> findById(UUID id) {
        return jpaRepository.findById(id.toString()).map(mapper::toDomain);
    }

    @Override
    public Page<Category> findAllByUserId(UUID userId, String search, Pageable pageable) {
        long total = jpaRepository.countWithFilters(userId.toString(), search);
        List<String> pageIds = PaginationUtils.slicePage(jpaRepository.findPageIds(userId.toString(), search), pageable);

        List<Category> categories = pageIds.isEmpty()
                ? List.of()
                : jpaRepository.findAllByIds(pageIds)
                .stream()
                .map(mapper::toDomain)
                .toList();

        return new PageImpl<>(categories, pageable, total);
    }

    @Override
    public List<Category> getSelector(UUID userId) {
        return jpaRepository.findAllByUserId(userId.toString())
                .stream()
                .map(mapper::toDomain)
                .toList();
    }

    @Override
    public void deleteById(UUID id) {
        jpaRepository.deleteById(id.toString());
    }

    @Override
    public boolean existsByUserIdAndName(UUID userId, String name) {
        return jpaRepository.existsByUserIdAndName(userId.toString(), name);
    }
}