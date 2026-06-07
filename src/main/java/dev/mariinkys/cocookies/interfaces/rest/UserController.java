package dev.mariinkys.cocookies.interfaces.rest;

import dev.mariinkys.cocookies.application.port.UserUseCase;
import dev.mariinkys.cocookies.application.service.RequesterContext;
import dev.mariinkys.cocookies.interfaces.dto.PageResponse;
import dev.mariinkys.cocookies.interfaces.dto.user.ChangePasswordRequest;
import dev.mariinkys.cocookies.interfaces.dto.user.UpdateRequest;
import dev.mariinkys.cocookies.interfaces.dto.user.UserResponse;
import jakarta.validation.Valid;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Sort;
import org.springframework.http.ResponseEntity;
import org.springframework.security.access.prepost.PreAuthorize;
import org.springframework.security.core.annotation.AuthenticationPrincipal;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.web.bind.annotation.*;

import java.util.UUID;

@RestController
@RequestMapping("/api/users")
public class UserController {

    private final UserUseCase userUseCase;

    public UserController(UserUseCase userUseCase) {
        this.userUseCase = userUseCase;
    }

    // ADMIN only
    @GetMapping
    public ResponseEntity<PageResponse<UserResponse>> getAll(
            @RequestParam(defaultValue = "0") int page,
            @RequestParam(defaultValue = "10") int size,
            @RequestParam(defaultValue = "createdAt") String sortBy,
            @RequestParam(defaultValue = "desc") String sortDir,
            @RequestParam(defaultValue = "") String search) {

        var direction = sortDir.equalsIgnoreCase("asc") ? Sort.Direction.ASC : Sort.Direction.DESC;
        var pageable = PageRequest.of(page, size, Sort.by(direction, sortBy));
        var result = userUseCase.getAllUsers(search, pageable).map(UserResponse::from);
        return ResponseEntity.ok(PageResponse.from(result));
    }

    // Gets only themselves (enforced in service)
    @GetMapping("/{id}")
    public ResponseEntity<UserResponse> getById(@PathVariable UUID id,
                                                @AuthenticationPrincipal UserDetails userDetails) {
        var user = userUseCase.getUserById(id, requesterContext(userDetails));
        return ResponseEntity.ok(UserResponse.from(user));
    }

    // Updates only themselves (enforced in service)
    @PutMapping("/{id}")
    public ResponseEntity<UserResponse> update(@PathVariable UUID id,
                                               @Valid @RequestBody UpdateRequest request,
                                               @AuthenticationPrincipal UserDetails userDetails) {
        var user = userUseCase.updateUser(id, request.name(), request.email(), requesterContext(userDetails));
        return ResponseEntity.ok(UserResponse.from(user));
    }

    @PatchMapping("/{id}/password")
    public ResponseEntity<Void> changePassword(@PathVariable UUID id,
                                               @Valid @RequestBody ChangePasswordRequest request,
                                               @AuthenticationPrincipal UserDetails userDetails) {
        userUseCase.changePassword(id, request.currentPassword(), request.newPassword(),
                requesterContext(userDetails));
        return ResponseEntity.noContent().build();
    }

    // User deletes only themselves (enforced in service)
    @DeleteMapping("/{id}")
    public ResponseEntity<Void> delete(@PathVariable UUID id,
                                       @AuthenticationPrincipal UserDetails userDetails) {
        userUseCase.deleteUser(id, requesterContext(userDetails));
        return ResponseEntity.noContent().build();
    }

    @GetMapping("/me")
    public ResponseEntity<UserResponse> getMe(@AuthenticationPrincipal UserDetails userDetails) {
        var user = userUseCase.getUserByEmail(userDetails.getUsername());
        return ResponseEntity.ok(UserResponse.from(user));
    }

    private RequesterContext requesterContext(UserDetails userDetails) {
        var user = userUseCase.getUserByEmail(userDetails.getUsername());
        return new RequesterContext(user.getId(), userDetails.getUsername());
    }
}