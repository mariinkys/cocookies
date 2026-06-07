package dev.mariinkys.cocookies.infrastructure.persistence;

import dev.mariinkys.cocookies.domain.models.Difficulty;
import dev.mariinkys.cocookies.domain.repository.DifficultyRepository;
import dev.mariinkys.cocookies.infrastructure.persistence.mapper.DifficultyMapper;
import dev.mariinkys.cocookies.infrastructure.persistence.repository.DifficultyJpaRepository;
import dev.mariinkys.cocookies.infrastructure.utils.PaginationUtils;
import org.springframework.data.domain.Page;
import org.springframework.data.domain.PageImpl;
import org.springframework.data.domain.Pageable;
import org.springframework.stereotype.Component;

import java.util.List;
import java.util.Optional;
import java.util.UUID;

@Component
public class DifficultyRepositoryImpl implements DifficultyRepository {

    private final DifficultyJpaRepository jpaRepository;
    private final DifficultyMapper mapper;

    public DifficultyRepositoryImpl(DifficultyJpaRepository jpaRepository, DifficultyMapper mapper) {
        this.jpaRepository = jpaRepository;
        this.mapper = mapper;
    }

    @Override
    public Difficulty save(Difficulty difficulty) {
        return mapper.toDomain(jpaRepository.save(mapper.toEntity(difficulty)));
    }

    @Override
    public Optional<Difficulty> findById(UUID id) {
        return jpaRepository.findById(id.toString()).map(mapper::toDomain);
    }

    @Override
    public Page<Difficulty> findAllByUserId(UUID userId, String search, Pageable pageable) {
        long total = jpaRepository.countWithFilters(userId.toString(), search);
        List<String> pageIds = PaginationUtils.slicePage(
                jpaRepository.findPageIds(userId.toString(), search), pageable);

        List<Difficulty> difficulties = pageIds.isEmpty()
                ? List.of()
                : jpaRepository.findAllByIds(pageIds)
                .stream()
                .map(mapper::toDomain)
                .toList();

        return new PageImpl<>(difficulties, pageable, total);
    }

    @Override
    public List<Difficulty> getSelector(UUID userId) {
        return jpaRepository.findAllByUserId(userId.toString())
                .stream()
                .map(mapper::toDomain)
                .toList();
    }

    @Override
    public void deleteById(UUID id) {
        jpaRepository.deleteById(id.toString());
    }

    @Override
    public boolean existsByUserIdAndName(UUID userId, String name) {
        return jpaRepository.existsByUserIdAndName(userId.toString(), name);
    }
}