package dev.mariinkys.cocookies.infrastructure.persistence;

import dev.mariinkys.cocookies.domain.models.Recipe;
import dev.mariinkys.cocookies.domain.repository.RecipeRepository;
import dev.mariinkys.cocookies.infrastructure.persistence.entity.CategoryJpaEntity;
import dev.mariinkys.cocookies.infrastructure.persistence.entity.DifficultyJpaEntity;
import dev.mariinkys.cocookies.infrastructure.persistence.entity.RecipeJpaEntity;
import dev.mariinkys.cocookies.infrastructure.persistence.mapper.RecipeMapper;
import dev.mariinkys.cocookies.infrastructure.persistence.repository.CategoryJpaRepository;
import dev.mariinkys.cocookies.infrastructure.persistence.repository.DifficultyJpaRepository;
import dev.mariinkys.cocookies.infrastructure.persistence.repository.RecipeJpaRepository;
import dev.mariinkys.cocookies.infrastructure.utils.PaginationUtils;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.PageImpl;
import org.springframework.data.domain.Pageable;
import org.springframework.stereotype.Component;

import java.util.List;
import java.util.Optional;
import java.util.UUID;

@Component
public class RecipeRepositoryImpl implements RecipeRepository {

    private final RecipeJpaRepository jpaRepository;
    private final CategoryJpaRepository categoryJpaRepository;
    private final DifficultyJpaRepository difficultyJpaRepository;
    private final RecipeMapper mapper;

    public RecipeRepositoryImpl(RecipeJpaRepository jpaRepository,
                                CategoryJpaRepository categoryJpaRepository,
                                DifficultyJpaRepository difficultyJpaRepository,
                                RecipeMapper mapper) {
        this.jpaRepository = jpaRepository;
        this.categoryJpaRepository = categoryJpaRepository;
        this.difficultyJpaRepository = difficultyJpaRepository;
        this.mapper = mapper;
    }

    @Override
    public Recipe save(Recipe recipe) {
        CategoryJpaEntity categoryEntity = recipe.getCategory() != null
                ? categoryJpaRepository.getReferenceById(recipe.getCategory().getId().toString())
                : null;
        DifficultyJpaEntity difficultyEntity = recipe.getDifficulty() != null
                ? difficultyJpaRepository.getReferenceById(recipe.getDifficulty().getId().toString())
                : null;

        RecipeJpaEntity entity = mapper.toEntity(recipe, categoryEntity, difficultyEntity);

        // If updating, load the existing entity and replace collections in-place so Hibernate deletes old rows before inserting new ones
        jpaRepository.findById(recipe.getId().toString()).ifPresent(existing -> {
            existing.getSteps().clear();
            existing.getIngredients().clear();
            existing.setNutrition(null);
            jpaRepository.saveAndFlush(existing);
        });

        return mapper.toDomain(jpaRepository.save(entity));
    }

    @Override
    public Optional<Recipe> findById(UUID id) {
        return jpaRepository.findByIdWithDetails(id.toString()).map(mapper::toDomain);
    }


    @Override
    public Page<Recipe> findAllByUserId(UUID userId, String search, Pageable pageable) {
        long total = jpaRepository.countWithFilters(userId.toString(), search);
        List<String> pageIds = PaginationUtils.slicePage(jpaRepository.findPageIds(userId.toString(), search), pageable);

        List<Recipe> recipes = pageIds.isEmpty()
                ? List.of()
                : jpaRepository.findAllByIdsWithDetails(pageIds, pageable.getSort())
                .stream()
                .map(mapper::toDomain)
                .toList();

        return new PageImpl<>(recipes, pageable, total);
    }

    @Override
    public Page<Recipe> findAllShared(String search, Pageable pageable) {
        long total = jpaRepository.countSharedWithFilters(search);

        List<String> allIds = jpaRepository.findSharedPageIds(search);
        int start = (int) pageable.getOffset();
        int end = Math.min(start + pageable.getPageSize(), allIds.size());
        List<String> pageIds = start >= allIds.size() ? List.of() : allIds.subList(start, end);

        List<Recipe> recipes = pageIds.isEmpty()
                ? List.of()
                : jpaRepository.findAllByIdsWithDetails(pageIds, pageable.getSort())
                .stream()
                .map(mapper::toDomain)
                .toList();

        return new PageImpl<>(recipes, pageable, total);
    }

    @Override
    public void deleteById(UUID id) {
        jpaRepository.deleteById(id.toString());
    }
}