package dev.mariinkys.cocookies.infrastructure.persistence.repository;

import dev.mariinkys.cocookies.infrastructure.persistence.entity.UserJpaEntity;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.data.jpa.repository.JpaRepository;
import org.springframework.data.jpa.repository.Query;
import org.springframework.data.repository.query.Param;

import java.util.Optional;
import java.util.UUID;

public interface UserJpaRepository extends JpaRepository<UserJpaEntity, String> {
    boolean existsByEmail(String email);
    Optional<UserJpaEntity> findByEmail(String email);

    @Query("""
    SELECT u FROM UserJpaEntity u
    WHERE (:search IS NULL OR :search = ''
        OR LOWER(u.name)  LIKE LOWER(CONCAT('%', :search, '%'))
        OR LOWER(u.email) LIKE LOWER(CONCAT('%', :search, '%')))
    """)
    Page<UserJpaEntity> findAll(@Param("search") String search, Pageable pageable);
}
