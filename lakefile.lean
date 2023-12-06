import Lake
open Lake DSL

package «AoC» {
  -- add package configuration options here
}

lean_lib «AoC» {
  -- add library configuration options here
}

@[default_target]
lean_exe «AoC» {
  root := `Main
}
