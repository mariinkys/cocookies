package dev.mariinkys.cocookies.interfaces.rest;

import dev.mariinkys.cocookies.application.port.DifficultyUseCase;
import dev.mariinkys.cocookies.application.port.UserUseCase;
import dev.mariinkys.cocookies.application.service.RequesterContext;
import dev.mariinkys.cocookies.interfaces.dto.PageResponse;
import dev.mariinkys.cocookies.interfaces.dto.difficulty.DifficultyRequest;
import dev.mariinkys.cocookies.interfaces.dto.difficulty.DifficultyResponse;
import jakarta.validation.Valid;
import org.springframework.data.domain.PageRequest;
import org.springframework.data.domain.Sort;
import org.springframework.http.HttpStatus;
import org.springframework.http.ResponseEntity;
import org.springframework.security.core.annotation.AuthenticationPrincipal;
import org.springframework.security.core.userdetails.UserDetails;
import org.springframework.web.bind.annotation.*;

import java.util.List;
import java.util.UUID;

@RestController
@RequestMapping("/api/difficulties")
public class DifficultyController {

    private final DifficultyUseCase difficultyUseCase;
    private final UserUseCase userUseCase;

    public DifficultyController(DifficultyUseCase difficultyUseCase, UserUseCase userUseCase) {
        this.difficultyUseCase = difficultyUseCase;
        this.userUseCase = userUseCase;
    }

    @GetMapping
    public ResponseEntity<PageResponse<DifficultyResponse>> getAll(
            @RequestParam(defaultValue = "0") int page,
            @RequestParam(defaultValue = "10") int size,
            @RequestParam(defaultValue = "name") String sortBy,
            @RequestParam(defaultValue = "asc") String sortDir,
            @RequestParam(defaultValue = "") String search,
            @AuthenticationPrincipal UserDetails requester) {

        var direction = sortDir.equalsIgnoreCase("asc") ? Sort.Direction.ASC : Sort.Direction.DESC;
        var pageable = PageRequest.of(page, size, Sort.by(direction, sortBy));
        var result = difficultyUseCase.getAllDifficulties(search, pageable, requesterContext(requester))
                .map(DifficultyResponse::from);
        return ResponseEntity.ok(PageResponse.from(result));
    }

    @GetMapping("selector")
    public ResponseEntity<List<DifficultyResponse>> getSelector(@AuthenticationPrincipal UserDetails requester) {
        var result = difficultyUseCase.getSelector(requesterContext(requester)).stream()
                .map(DifficultyResponse::from).toList();

        return ResponseEntity.ok(result);
    }

    @GetMapping("/{id}")
    public ResponseEntity<DifficultyResponse> getById(@PathVariable UUID id, @AuthenticationPrincipal UserDetails requester) {
        return ResponseEntity.ok(DifficultyResponse.from(difficultyUseCase.getDifficultyById(id, requesterContext(requester))));
    }

    @PostMapping
    public ResponseEntity<DifficultyResponse> create(
            @Valid @RequestBody DifficultyRequest request,
            @AuthenticationPrincipal UserDetails requester) {

        var difficulty = difficultyUseCase.createDifficulty(
                request.name(), request.description(), requesterContext(requester)
        );
        return ResponseEntity.status(HttpStatus.CREATED).body(DifficultyResponse.from(difficulty));
    }

    @PutMapping("/{id}")
    public ResponseEntity<DifficultyResponse> update(
            @PathVariable UUID id,
            @Valid @RequestBody DifficultyRequest request,
            @AuthenticationPrincipal UserDetails requester) {

        var difficulty = difficultyUseCase.updateDifficulty(id, request.name(), request.description(), requesterContext(requester));
        return ResponseEntity.ok(DifficultyResponse.from(difficulty));
    }

    @DeleteMapping("/{id}")
    public ResponseEntity<Void> delete(@PathVariable UUID id, @AuthenticationPrincipal UserDetails requester) {
        difficultyUseCase.deleteDifficulty(id, requesterContext(requester));
        return ResponseEntity.noContent().build();
    }

    private RequesterContext requesterContext(UserDetails userDetails) {
        var user = userUseCase.getUserByEmail(userDetails.getUsername());
        return new RequesterContext(user.getId(), userDetails.getUsername());
    }
}