# Stage BUILD
FROM node:18-alpine as builder

# Build-time variables
ARG backend_host=http://localhost:8000
# Also available at run-time
ENV ORIGIN=$backend_host

WORKDIR /app
COPY . ./

RUN npm ci --no-audit
RUN npm run build

# Stage RUN
FROM node:18-alpine

WORKDIR /app

COPY --from=builder /app/package*.json ./
COPY --from=builder /app/build ./

# clean install dependencies, no devDependencies, no prepare script
RUN npm ci --omit=dev --ignore-scripts --no-audit

EXPOSE 3000
CMD ["node", "./index.js"]
