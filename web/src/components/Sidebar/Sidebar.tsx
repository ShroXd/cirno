import { Cog6ToothIcon, MagnifyingGlassIcon } from "@heroicons/react/20/solid";
import {
  Card,
  Typography,
  List,
  ListItem,
  ListItemPrefix,
  Input,
} from "@material-tailwind/react";
import { useTranslation } from "react-i18next";
import { NavLink } from "react-router-dom";

export const Sidebar = () => {
  const { t } = useTranslation();

  return (
    <Card className="w-min-72 h-[calc(100vh-2rem)] max-w-[20rem] p-4 shadow-xl shadow-blue-gray-900/15">
      <div className="mb-2 flex items-center gap-4 p-4">
        <img
          src="https://docs.material-tailwind.com/img/logo-ct-dark.png"
          alt="brand"
          className="h-8 w-8"
        />
        <Typography variant="h5" color="blue-gray">
          Sidebar
        </Typography>
      </div>
      <div className="p-2">
        <Input
          icon={<MagnifyingGlassIcon className="h-5 w-5" />}
          label="Search"
          crossOrigin="anonymous"
        />
      </div>
      <List>
        <NavLink to="/setting">
          <ListItem>
            <ListItemPrefix>
              <Cog6ToothIcon className="h-5 w-5" />
            </ListItemPrefix>
            {t("sidebar.settings")}
          </ListItem>
        </NavLink>
      </List>
    </Card>
  );
};
