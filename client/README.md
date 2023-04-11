## Development

Once you've created a project and installed dependencies with `npm install` or `yarn install`, start a development server:

```bash
npm run dev
```

or

```bash
yarn dev
```

## Building

To create a production version of your app:

```bash
npm run build
```

or

```bash
yarn build
```

You can preview the production build with `npm run preview` or `yarn preview`.

### Environment

- Environment variables can be defined by .env file in the root folder of client. A .env.example file is commited to the repository. Also, npm dev script will check if you have .env file created, if not then it will copy the example file for you, to make development quicker.
  - Feel free to modify your .env file to your liking, it was added to .gitignore for a reason.
  - If you need a new environment configuration, do not forget to add it to the example file and commit it.
- You can set variables in any other know way.
