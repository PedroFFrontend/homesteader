# FROM node:22-alpine AS builder
# WORKDIR /app
# COPY package*.json ./
# # RUN npm install -y
# RUN npm ci
# COPY . .
# RUN npm run build
# RUN npm prune --production

# FROM node:22-alpine
# WORKDIR /app
# COPY --from=builder /app/build build/
# COPY --from=builder /app/node_modules node_modules/
# COPY package.json .
# EXPOSE 3000
# ENV NODE_ENV=production
# CMD [ "node", "build" ]


# Our Node base image
FROM node:22-alpine
# Set the Node environment to development to ensure all packages are installed
ENV NODE_ENV development
# Change our current working directory
RUN mkdir -p /usr/src/app
WORKDIR /usr/src/app
# Copy over `package.json` and lock files to optimize the build process
COPY ["package.json", "package-lock.json*", "npm-shrinkwrap.json*", "yarn.lock", "./"]
# Install Node modules
RUN yarn
# Copy over rest of the project files
COPY . /usr/src/app
# Expose port 3000 for the SvelteKit app and 24678 for Vite's HMR
EXPOSE 3000
EXPOSE 24678
EXPOSE 5173
# Run `yarn dev` and set the host to 0.0.0.0 so we can access the web app from outside
CMD ["yarn", "dev", "--host", "0.0.0.0"]