#!/usr/bin/env node

import { ClientConfigBuilder, PolywrapClient } from "@polywrap/client-js";
import { PluginPackage } from "@polywrap/plugin-js";

(async () => {
  const client = new PolywrapClient(
    new ClientConfigBuilder().addDefaults()
      .addPackage("test/a", PluginPackage.from((module: any) => ({
        "meth": (args: any) => {
          console.log(args);
          return true;
        }
      })))
      .build()
  );

  const result = await client.invoke({
    uri: "fs/../wrap/bin",
    method: "eval",
    args: {
      src: `
      const doStuff4 = () => {
        return __wrap_subinvoke("test/a", "meth", { message: "a" });
      };

      const doStuff3 = () => {
        return doStuff4();
      };
      const doStuff2 = () => {
        return doStuff3();
      };
      const doStuff1 = () => {
        return doStuff2();
      };
      const doStuff = () => {
        return doStuff1();
      };
      const alert = () => {
        return  doStuff();
      };
      
      alert();
      `
    }
  });

  console.log(result);
})();
