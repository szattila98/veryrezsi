# Stage BUILD
FROM node:18-alpine

WORKDIR /app
COPY . .

RUN npm ci --no-audit
RUN npm run build

# Stage RUN
FROM node:18-alpine

WORKDIR /app

COPY --from=0 /app/package*.json .
COPY --from=0 /app/build .

# clean install dependencies, no devDependencies, no prepare script
RUN npm ci --production --ignore-scripts --no-audit

EXPOSE 3000
CMD ["node", "./index.js"]
