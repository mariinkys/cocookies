package dev.mariinkys.cocookies.infrastructure.persistence.repository;

import dev.mariinkys.cocookies.infrastructure.persistence.entity.CategoryJpaEntity;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;

import java.util.List;

public interface CategoryJpaRepository extends JpaRepository<CategoryJpaEntity, String> {

    boolean existsByUserIdAndName(String userId, String name);

    // Step 1 — matching IDs only for pagination
    @Query("""
        SELECT c.id FROM CategoryJpaEntity c
        WHERE c.userId = :userId
        AND (:search IS NULL OR :search = ''
            OR LOWER(c.name)        LIKE LOWER(CONCAT('%', :search, '%'))
            OR LOWER(c.description) LIKE LOWER(CONCAT('%', :search, '%')))
        ORDER BY c.name ASC
        """)
    List<String> findPageIds(@Param("userId") String userId,
                           @Param("search") String search);

    // Step 2 — full entities for the given IDs
    @Query("""
        SELECT c FROM CategoryJpaEntity c
        WHERE c.id IN :ids
        ORDER BY c.name ASC
        """)
    List<CategoryJpaEntity> findAllByIds(@Param("ids") List<String> ids);

    // Count query for pagination total
    @Query("""
        SELECT COUNT(c) FROM CategoryJpaEntity c
        WHERE c.userId = :userId
        AND (:search IS NULL OR :search = ''
            OR LOWER(c.name)        LIKE LOWER(CONCAT('%', :search, '%'))
            OR LOWER(c.description) LIKE LOWER(CONCAT('%', :search, '%')))
        """)
    long countWithFilters(@Param("userId") String userId,
                          @Param("search") String search);

    List<CategoryJpaEntity> findAllByUserId(String userId);
}