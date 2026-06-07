# Build Stage
FROM docker.io/library/maven:3.9-eclipse-temurin-26 AS build
WORKDIR /app
COPY pom.xml .

RUN mvn dependency:go-offline -B
COPY src ./src
RUN mvn package -DskipTests -B

# Runtime Stage
FROM docker.io/library/eclipse-temurin:26-jre-alpine
WORKDIR /app
COPY --from=build /app/target/*.jar app.jar

# Non-root user
RUN addgroup -S appgroup && adduser -S appuser -G appgroup
USER appuser

EXPOSE 8080
ENTRYPOINT ["java", "--enable-native-access=ALL-UNNAMED", "-jar", "app.jar"]