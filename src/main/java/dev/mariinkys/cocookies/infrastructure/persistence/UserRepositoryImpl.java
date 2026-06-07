package dev.mariinkys.cocookies.infrastructure.persistence;

import dev.mariinkys.cocookies.domain.models.User;
import dev.mariinkys.cocookies.domain.repository.UserRepository;
import dev.mariinkys.cocookies.infrastructure.persistence.mapper.UserMapper;
import dev.mariinkys.cocookies.infrastructure.persistence.repository.UserJpaRepository;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.stereotype.Component;

import java.util.Optional;
import java.util.UUID;

@Component
public class UserRepositoryImpl implements UserRepository {

    private final UserJpaRepository jpaRepository;
    private final UserMapper mapper;

    public UserRepositoryImpl(UserJpaRepository jpaRepository, UserMapper mapper) {
        this.jpaRepository = jpaRepository;
        this.mapper = mapper;
    }

    @Override
    public User save(User user) {
        var entity = mapper.toEntity(user);
        var saved = jpaRepository.save(entity);
        return mapper.toDomain(saved);
    }

    @Override
    public Optional<User> findById(UUID id) {
        return jpaRepository.findById(id.toString()).map(mapper::toDomain);
    }

    @Override
    public Optional<User> findByEmail(String email) { return jpaRepository.findByEmail(email).map(mapper::toDomain); }

    @Override
    public Page<User> findAll(String search, Pageable pageable) {
        return jpaRepository.findAll(search, pageable).map(mapper::toDomain);
    }

    @Override
    public void deleteById(UUID id) {
        jpaRepository.deleteById(id.toString());
    }

    @Override
    public boolean existsByEmail(String email) {
        return jpaRepository.existsByEmail(email);
    }
}