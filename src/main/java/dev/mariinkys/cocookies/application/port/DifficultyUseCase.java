package dev.mariinkys.cocookies.application.port;

import dev.mariinkys.cocookies.application.service.RequesterContext;
import dev.mariinkys.cocookies.domain.models.Difficulty;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;

import java.util.List;
import java.util.UUID;

public interface DifficultyUseCase {
    Difficulty createDifficulty(String name, String description, RequesterContext requester);
    Difficulty getDifficultyById(UUID id, RequesterContext requester);
    Page<Difficulty> getAllDifficulties(String search, Pageable pageable, RequesterContext requester);
    List<Difficulty> getSelector(RequesterContext requester);
    Difficulty updateDifficulty(UUID id, String name, String description, RequesterContext requester);
    void deleteDifficulty(UUID id, RequesterContext requester);
}