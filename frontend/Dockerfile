FROM node:18-alpine AS base
WORKDIR /src

FROM base AS deps
COPY package.json pnpm-lock.yaml ./
RUN corepack pnpm i --frozen-lockfile


FROM base AS builder
COPY --from=deps /src/node_modules ./node_modules
COPY . .
RUN corepack pnpm build

FROM base AS runner
ENV NODE_ENV production
COPY --from=builder /src/public ./public
COPY --from=builder /src/.next/standalone ./
COPY --from=builder /src/.next/static ./.next/static
EXPOSE 3000
CMD ["node", "server.js"]
