# Stage BUILD
FROM mhart/alpine-node:16

WORKDIR /app
COPY package.json package-lock.json ./
RUN npm ci

COPY . .

RUN npm run build

# Stage RUN
### slim: Smaller and Safer
FROM mhart/alpine-node:slim-16

WORKDIR /app

COPY --from=0 /app/package*.json ./

# clean install dependencies, no devDependencies, no prepare script
RUN npm ci --production --ignore-scripts

COPY --from=0 /app/build ./

EXPOSE 3000
CMD ["node", "./index.js"]