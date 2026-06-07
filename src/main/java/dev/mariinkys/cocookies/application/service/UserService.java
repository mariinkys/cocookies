package dev.mariinkys.cocookies.application.service;

import dev.mariinkys.cocookies.application.exception.EmailAlreadyInUseException;
import dev.mariinkys.cocookies.application.exception.InvalidPasswordException;
import dev.mariinkys.cocookies.application.exception.UserNotFoundException;
import dev.mariinkys.cocookies.application.port.PasswordHasher;
import dev.mariinkys.cocookies.domain.models.User;
import dev.mariinkys.cocookies.domain.repository.UserRepository;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.security.access.AccessDeniedException;
import org.springframework.stereotype.Service;

import java.util.UUID;
import dev.mariinkys.cocookies.application.port.UserUseCase;
import org.springframework.transaction.annotation.Transactional;

@Service
public class UserService implements UserUseCase {

    private final UserRepository userRepository;
    private final PasswordHasher passwordHasher;

    public UserService(UserRepository userRepository, PasswordHasher passwordHasher) {
        this.userRepository = userRepository;
        this.passwordHasher = passwordHasher;
    }

    @Override
    @Transactional
    public void createUser(String name, String email, String rawPassword) {
        if (userRepository.existsByEmail(email)) {
            throw new EmailAlreadyInUseException(email);
        }
        userRepository.save(new User(name, email, passwordHasher.hash(rawPassword)));
    }

    @Override
    @Transactional(readOnly = true)
    public User getUserById(UUID id, RequesterContext requester) {
        User target = userRepository.findById(id)
                .orElseThrow(() -> new UserNotFoundException(id));
        if (!target.getId().equals(requester.id())) {
            throw new AccessDeniedException("You can only access your own account");
        }
        return target;
    }

    @Override
    @Transactional(readOnly = true)
    public User getUserByEmail(String email) {
        return userRepository.findByEmail(email)
                .orElseThrow(() -> new UserNotFoundException(email));
    }

    @Override
    @Transactional(readOnly = true)
    public Page<User> getAllUsers(String search, Pageable pageable) {
        return userRepository.findAll(search, pageable);
    }

    @Override
    @Transactional
    public User updateUser(UUID id, String name, String email, RequesterContext requester) {
        User target = userRepository.findById(id)
                .orElseThrow(() -> new UserNotFoundException(id));
        if (!target.getId().equals(requester.id())) {
            throw new AccessDeniedException("You can only update your own account");
        }
        return userRepository.save(target.withUpdatedDetails(name, email));
    }

    @Override
    @Transactional
    public void changePassword(UUID id, String currentPassword, String newPassword,
                               RequesterContext requester) {
        User target = userRepository.findById(id)
                .orElseThrow(() -> new UserNotFoundException(id));

        if (!target.getId().equals(requester.id())) {
            throw new AccessDeniedException("You can only change your own password");
        }
        if (currentPassword == null || currentPassword.isBlank()) {
            throw new InvalidPasswordException();
        }
        if (!passwordHasher.matches(currentPassword, target.getPassword())) {
            throw new InvalidPasswordException();
        }


        userRepository.save(target.withPassword(passwordHasher.hash(newPassword)));
    }

    @Override
    @Transactional
    public void deleteUser(UUID id, RequesterContext requester) {
        User target = userRepository.findById(id)
                .orElseThrow(() -> new UserNotFoundException(id));
        if (!target.getId().equals(requester.id())) {
            throw new AccessDeniedException("You can only delete your own account");
        }
        userRepository.deleteById(id);
    }
}