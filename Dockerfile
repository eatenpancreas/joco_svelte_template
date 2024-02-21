FROM node:latest

WORKDIR /app
COPY package.json /app/package.json
RUN npm install
COPY . /app
COPY .env.production /app/.env

RUN npm run build
RUN echo "gay"
CMD ["node", "build"]
