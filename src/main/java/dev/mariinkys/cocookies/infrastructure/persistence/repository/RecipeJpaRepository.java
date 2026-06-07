package dev.mariinkys.cocookies.infrastructure.persistence.repository;

import dev.mariinkys.cocookies.infrastructure.persistence.entity.RecipeJpaEntity;
import org.springframework.data.domain.Sort;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;

import java.util.List;
import java.util.Optional;

public interface RecipeJpaRepository extends JpaRepository<RecipeJpaEntity, String> {

    // Single recipe with all associations loaded
    @Query("""
        SELECT r FROM RecipeJpaEntity r
        LEFT JOIN FETCH r.category
        LEFT JOIN FETCH r.difficulty
        LEFT JOIN FETCH r.steps
        LEFT JOIN FETCH r.ingredients
        LEFT JOIN FETCH r.nutrition
        WHERE r.id = :id
        """)
    Optional<RecipeJpaEntity> findByIdWithDetails(@Param("id") String id);

    // Step 1 — matching IDs for user-scoped pagination
    @Query("""
        SELECT r.id FROM RecipeJpaEntity r
        WHERE r.userId = :userId
        AND (:search IS NULL OR :search = ''
            OR LOWER(r.title)       LIKE LOWER(CONCAT('%', :search, '%'))
            OR LOWER(r.description) LIKE LOWER(CONCAT('%', :search, '%')))
        """)
    List<String> findPageIds(@Param("userId") String userId, @Param("search") String search);

    // Step 1 — matching IDs for shared recipes pagination
    @Query("""
        SELECT r.id FROM RecipeJpaEntity r
        WHERE r.shared = true
        AND (:search IS NULL OR :search = ''
            OR LOWER(r.title)       LIKE LOWER(CONCAT('%', :search, '%'))
            OR LOWER(r.description) LIKE LOWER(CONCAT('%', :search, '%')))
        """)
    List<String> findSharedPageIds(@Param("search") String search);

    // Step 2 — full entities with associations for the given IDs
    @Query("""
        SELECT DISTINCT r FROM RecipeJpaEntity r
        LEFT JOIN FETCH r.category
        LEFT JOIN FETCH r.difficulty
        LEFT JOIN FETCH r.steps
        LEFT JOIN FETCH r.ingredients
        LEFT JOIN FETCH r.nutrition
        WHERE r.id IN :ids
        """)
    List<RecipeJpaEntity> findAllByIdsWithDetails(@Param("ids") List<String> ids, Sort sort);

    // Count query for user-scoped pagination
    @Query("""
        SELECT COUNT(r) FROM RecipeJpaEntity r
        WHERE r.userId = :userId
        AND (:search IS NULL OR :search = ''
            OR LOWER(r.title)       LIKE LOWER(CONCAT('%', :search, '%'))
            OR LOWER(r.description) LIKE LOWER(CONCAT('%', :search, '%')))
        """)
    long countWithFilters(@Param("userId") String userId,
                          @Param("search") String search);

    // Count query for shared recipes pagination
    @Query("""
        SELECT COUNT(r) FROM RecipeJpaEntity r
        WHERE r.shared = true
        AND (:search IS NULL OR :search = ''
            OR LOWER(r.title)       LIKE LOWER(CONCAT('%', :search, '%'))
            OR LOWER(r.description) LIKE LOWER(CONCAT('%', :search, '%')))
        """)
    long countSharedWithFilters(@Param("search") String search);
}