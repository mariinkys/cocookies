package dev.mariinkys.cocookies.infrastructure.persistence.repository;

import dev.mariinkys.cocookies.infrastructure.persistence.entity.DifficultyJpaEntity;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;

import java.util.List;

public interface DifficultyJpaRepository extends JpaRepository<DifficultyJpaEntity, String> {

    boolean existsByUserIdAndName(String userId, String name);

    // Step 1 — matching IDs only for pagination
    @Query("""
        SELECT d.id FROM DifficultyJpaEntity d
        WHERE d.userId = :userId
        AND (:search IS NULL OR :search = ''
            OR LOWER(d.name)        LIKE LOWER(CONCAT('%', :search, '%'))
            OR LOWER(d.description) LIKE LOWER(CONCAT('%', :search, '%')))
        ORDER BY d.name ASC
        """)
    List<String> findPageIds(@Param("userId") String userId,
                           @Param("search") String search);

    // Step 2 — full entities for the given IDs
    @Query("""
        SELECT d FROM DifficultyJpaEntity d
        WHERE d.id IN :ids
        ORDER BY d.name ASC
        """)
    List<DifficultyJpaEntity> findAllByIds(@Param("ids") List<String> ids);

    // Count query for pagination total
    @Query("""
        SELECT COUNT(d) FROM DifficultyJpaEntity d
        WHERE d.userId = :userId
        AND (:search IS NULL OR :search = ''
            OR LOWER(d.name)        LIKE LOWER(CONCAT('%', :search, '%'))
            OR LOWER(d.description) LIKE LOWER(CONCAT('%', :search, '%')))
        """)
    long countWithFilters(@Param("userId") String userId,
                          @Param("search") String search);

    List<DifficultyJpaEntity> findAllByUserId(String userId);
}