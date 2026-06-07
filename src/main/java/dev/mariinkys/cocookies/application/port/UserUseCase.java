package dev.mariinkys.cocookies.application.port;

import dev.mariinkys.cocookies.application.service.RequesterContext;
import dev.mariinkys.cocookies.domain.models.User;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;

import java.util.UUID;

public interface UserUseCase {
    void createUser(String name, String email, String rawPassword);
    User getUserById(UUID id, RequesterContext requester);
    User getUserByEmail(String email);
    Page<User> getAllUsers(String search, Pageable pageable);
    User updateUser(UUID id, String name, String email, RequesterContext requester);
    void changePassword(UUID id, String currentPassword, String newPassword, RequesterContext requester);
    void deleteUser(UUID id, RequesterContext requester);
}
