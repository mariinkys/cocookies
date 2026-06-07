package dev.mariinkys.cocookies.infrastructure.utils;

import org.springframework.data.domain.Pageable;
import java.util.List;

public class PaginationUtils {

    private PaginationUtils() {}

    public static <T> List<T> slicePage(List<T> allIds, Pageable pageable) {
        int start = (int) pageable.getOffset();
        int end = Math.min(start + pageable.getPageSize(), allIds.size());
        return start >= allIds.size() ? List.of() : allIds.subList(start, end);
    }
}
