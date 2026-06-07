package dev.mariinkys.cocookies.application.service;

import dev.mariinkys.cocookies.application.exception.DifficultyNotFoundException;
import dev.mariinkys.cocookies.application.port.DifficultyUseCase;
import dev.mariinkys.cocookies.domain.models.Difficulty;
import dev.mariinkys.cocookies.domain.repository.DifficultyRepository;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.Pageable;
import org.springframework.security.access.AccessDeniedException;
import org.springframework.stereotype.Service;
import org.springframework.transaction.annotation.Transactional;

import java.util.List;
import java.util.UUID;

@Service
public class DifficultyService implements DifficultyUseCase {

    private final DifficultyRepository difficultyRepository;

    public DifficultyService(DifficultyRepository difficultyRepository) {
        this.difficultyRepository = difficultyRepository;
    }

    @Override
    @Transactional
    public Difficulty createDifficulty(String name, String description, RequesterContext requester) {
        if (difficultyRepository.existsByUserIdAndName(requester.id(), name))
            throw new IllegalArgumentException("Difficulty with name '" + name + "' already exists");

        return difficultyRepository.save(new Difficulty(requester.id(), name, description));
    }

    @Override
    @Transactional(readOnly = true)
    public Difficulty getDifficultyById(UUID id, RequesterContext requester) {
        Difficulty target = difficultyRepository.findById(id)
                .orElseThrow(() -> new DifficultyNotFoundException(id));

        if (!target.getId().equals(requester.id())) {
            throw new AccessDeniedException("You can only access your own difficulty");
        }

        return target;
    }

    @Override
    @Transactional(readOnly = true)
    public Page<Difficulty> getAllDifficulties(String search, Pageable pageable, RequesterContext requester) {
        return difficultyRepository.findAllByUserId(requester.id(), search, pageable);
    }

    @Override
    @Transactional(readOnly = true)
    public List<Difficulty> getSelector(RequesterContext requester) {
        return difficultyRepository.getSelector(requester.id());
    }

    @Override
    @Transactional
    public Difficulty updateDifficulty(UUID id, String name, String description, RequesterContext requester) {
        Difficulty existing = getDifficultyById(id, requester);

        if (!existing.getName().equals(name) &&
                difficultyRepository.existsByUserIdAndName(existing.getUserId(), name))
            throw new IllegalArgumentException("Difficulty with name '" + name + "' already exists");

        return difficultyRepository.save(existing.withUpdatedDetails(name, description));
    }

    @Override
    @Transactional
    public void deleteDifficulty(UUID id, RequesterContext requester) {
        getDifficultyById(id, requester);
        difficultyRepository.deleteById(id);
    }
}