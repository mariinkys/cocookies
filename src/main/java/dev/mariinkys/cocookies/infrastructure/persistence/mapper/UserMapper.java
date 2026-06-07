package dev.mariinkys.cocookies.infrastructure.persistence.mapper;

import dev.mariinkys.cocookies.domain.models.User;
import dev.mariinkys.cocookies.infrastructure.persistence.entity.UserJpaEntity;
import org.springframework.stereotype.Component;

import java.util.UUID;

@Component
public class UserMapper {

    public User toDomain(UserJpaEntity entity) {
        return new User(
                UUID.fromString(entity.getId()),
                entity.getName(),
                entity.getEmail(),
                entity.getPassword(),
                entity.getFailedLoginAttempts(),
                entity.getLockedUntil(),
                entity.getCreatedAt(),
                entity.getUpdatedAt()
        );
    }

    public UserJpaEntity toEntity(User user) {
        return new UserJpaEntity(
                user.getId().toString(),
                user.getName(),
                user.getEmail(),
                user.getPassword(),
                user.getFailedLoginAttempts(),
                user.getLockedUntil(),
                user.getCreatedAt(),
                user.getUpdatedAt()
        );
    }
}