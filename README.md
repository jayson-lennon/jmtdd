A minimal sample web server which demos vertical slice architecture and TDD.

## Notes

- Features are broken up into the `src/feat` folder. Everything needed for the feature should exist in it's own folder.
- `ApplicationBuilder` should default to the production implementations of all services needed for the application.
  - Override services by implementing an extension trait. See `src/feat/foo/mod.rs:FooApplicationBuilderExt` for an example.
  - Disable a feature by switching out the default production one with a stub. Note that any features dependent on the disabled feature will panic, but tests should catch this.
- Routes are added to `feat/web/feature_router.rs:all_routes()`. If you forget to add the route, then you should get a failed test if you are following TDD.
