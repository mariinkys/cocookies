package dev.mariinkys.cocookies.domain.repository;

import dev.mariinkys.cocookies.domain.models.User;
import java.util.Optional;
import java.util.UUID;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;

public interface UserRepository {
    User save(User user);
    Optional<User> findById(UUID id);
    Optional<User> findByEmail(String email);
    Page<User> findAll(String search, Pageable pageable);
    void deleteById(UUID id);
    boolean existsByEmail(String email);
}