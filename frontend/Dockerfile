FROM node:18-alpine AS base
WORKDIR /src

FROM base AS deps
COPY package.json pnpm-lock.yaml ./
RUN corepack pnpm i --frozen-lockfile


FROM base AS builder
COPY --from=deps /src/node_modules ./node_modules
COPY . .
# Uses a dummy secret to avoid a warning from Next.js. The actual secret is set at runtime.
RUN SESSION_SECRET=T3i4fOEJL1OYx/YsCgyv71dIE1uYGrlIhVukfN6cQaU= corepack pnpm build

FROM base AS runner
ENV NODE_ENV production
COPY --from=builder /src/public ./public
COPY --from=builder /src/.next/standalone ./
COPY --from=builder /src/.next/static ./.next/static
EXPOSE 3000
CMD ["node", "server.js"]
