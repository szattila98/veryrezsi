ARG client_url=http://localhost:3000
ARG backend_url=http://veryrezsi_server:8000/api

# Stage BUILD
FROM node:18-alpine as builder

# Build-time variables
ARG backend_url
ENV REZSI_API_BASE_URL=$backend_url

WORKDIR /app
COPY . ./

RUN npm ci --no-audit
RUN npm run build

# Stage RUN
FROM node:18-alpine

# Run-time variables
ARG client_url
ENV ORIGIN=$client_url

WORKDIR /app

COPY --from=builder /app/package*.json ./
COPY --from=builder /app/build ./

# clean install dependencies, no devDependencies, no prepare script
RUN npm ci --omit=dev --ignore-scripts --no-audit

EXPOSE 4000
CMD ["node", "./index.js"]
