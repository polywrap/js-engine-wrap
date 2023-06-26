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
      const doStuff = () => {
        return __wrap_subinvoke("test/a", "meth", { message: "a" });
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
