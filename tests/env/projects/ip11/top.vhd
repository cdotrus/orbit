library ieee;
use ieee.std_logic_1164.all;

library ip10;

entity top is
port (
  data: out std_logic
);
end entity;

architecture rtl of top is
begin

  u0: entity ip10.mid
    port map(
      data => data
    );

end architecture;