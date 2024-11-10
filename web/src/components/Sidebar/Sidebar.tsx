// import { createElement, useEffect } from "react";
// import { useLocation, useNavigate } from "react-router-dom";
// import {
//   FolderIcon,
//   Cog6ToothIcon,
//   SparklesIcon,
// } from "@heroicons/react/24/outline";
// import MovieIcon from "/movie.svg";

// export const Sidebar = () => {
//   const navigate = useNavigate();
//   const location = useLocation();

//   useEffect(() => {
//     console.log(location);
//   }, [location]);

//   const data = [
//     {
//       value: "library",
//       icon: FolderIcon,
//       path: "/",
//     },
//     {
//       value: "settings",
//       icon: Cog6ToothIcon,
//       path: "/settings",
//     },
//     {
//       value: "test",
//       icon: SparklesIcon,
//       path: "/test",
//     },
//   ];

//   return (
//     <div className="flex flex-col items-center gap-2 px-2 py-5 border w-20 border-gray-700 rounded-xl bg-gray-900 shadow-xl">
//       <img src={MovieIcon} alt="Movie Icon" className="w-11 h-11" />
//       <div className="flex flex-col items-center justify-center flex-1 gap-2">
//         {data.map(({ value, icon, path }) => (
//           <button
//             className={`px-3 py-3 rounded-xl focus:outline-none hover:border-transparent transition-all ease-in-out duration-300 shadow-inner s ${
//               location.pathname === path ? "bg-gray-800" : "bg-transparent"
//             }`}
//             key={value}
//             onClick={() => navigate(path)}
//           >
//             {createElement(icon, { className: "w-6 h-6 text-gray-50" })}
//           </button>
//         ))}
//       </div>
//     </div>
//   );
// };

import { useState } from 'react'
import {
  Card,
  Typography,
  List,
  ListItem,
  ListItemPrefix,
  ListItemSuffix,
  Chip,
  Accordion,
  AccordionHeader,
  AccordionBody,
  Input,
} from '@material-tailwind/react'
import {
  PresentationChartBarIcon,
  ShoppingBagIcon,
  UserCircleIcon,
  Cog6ToothIcon,
  InboxIcon,
  PowerIcon,
} from '@heroicons/react/24/solid'
import {
  ChevronRightIcon,
  ChevronDownIcon,
  MagnifyingGlassIcon,
} from '@heroicons/react/24/outline'

export const Sidebar = () => {
  const [open, setOpen] = useState(0)

  const handleOpen = (value: number) => {
    setOpen(open === value ? 0 : value)
  }

  return (
    <Card className='h-[calc(100vh-2rem)] w-full max-w-[20rem] p-4 shadow-xl shadow-blue-gray-900/5'>
      <div className='mb-2 flex items-center gap-4 p-4'>
        <img
          src='https://docs.material-tailwind.com/img/logo-ct-dark.png'
          alt='brand'
          className='h-8 w-8'
        />
        <Typography variant='h5' color='blue-gray'>
          Sidebar
        </Typography>
      </div>
      <div className='p-2'>
        <Input
          icon={<MagnifyingGlassIcon className='h-5 w-5' />}
          label='Search'
        />
      </div>
      <List>
        <Accordion
          open={open === 1}
          icon={
            <ChevronDownIcon
              strokeWidth={2.5}
              className={`mx-auto h-4 w-4 transition-transform ${open === 1 ? 'rotate-180' : ''}`}
            />
          }
        >
          <ListItem className='p-0' selected={open === 1}>
            <AccordionHeader
              onClick={() => handleOpen(1)}
              className='border-b-0 p-3'
            >
              <ListItemPrefix>
                <PresentationChartBarIcon className='h-5 w-5' />
              </ListItemPrefix>
              <Typography color='blue-gray' className='mr-auto font-normal'>
                Dashboard
              </Typography>
            </AccordionHeader>
          </ListItem>
          <AccordionBody className='py-1'>
            <List className='p-0'>
              <ListItem>
                <ListItemPrefix>
                  <ChevronRightIcon strokeWidth={3} className='h-3 w-5' />
                </ListItemPrefix>
                Analytics
              </ListItem>
              <ListItem>
                <ListItemPrefix>
                  <ChevronRightIcon strokeWidth={3} className='h-3 w-5' />
                </ListItemPrefix>
                Reporting
              </ListItem>
              <ListItem>
                <ListItemPrefix>
                  <ChevronRightIcon strokeWidth={3} className='h-3 w-5' />
                </ListItemPrefix>
                Projects
              </ListItem>
            </List>
          </AccordionBody>
        </Accordion>
        <Accordion
          open={open === 2}
          icon={
            <ChevronDownIcon
              strokeWidth={2.5}
              className={`mx-auto h-4 w-4 transition-transform ${open === 2 ? 'rotate-180' : ''}`}
            />
          }
        >
          <ListItem className='p-0' selected={open === 2}>
            <AccordionHeader
              onClick={() => handleOpen(2)}
              className='border-b-0 p-3'
            >
              <ListItemPrefix>
                <ShoppingBagIcon className='h-5 w-5' />
              </ListItemPrefix>
              <Typography color='blue-gray' className='mr-auto font-normal'>
                E-Commerce
              </Typography>
            </AccordionHeader>
          </ListItem>
          <AccordionBody className='py-1'>
            <List className='p-0'>
              <ListItem>
                <ListItemPrefix>
                  <ChevronRightIcon strokeWidth={3} className='h-3 w-5' />
                </ListItemPrefix>
                Orders
              </ListItem>
              <ListItem>
                <ListItemPrefix>
                  <ChevronRightIcon strokeWidth={3} className='h-3 w-5' />
                </ListItemPrefix>
                Products
              </ListItem>
            </List>
          </AccordionBody>
        </Accordion>
        <hr className='my-2 border-blue-gray-50' />
        <ListItem>
          <ListItemPrefix>
            <InboxIcon className='h-5 w-5' />
          </ListItemPrefix>
          Inbox
          <ListItemSuffix>
            <Chip
              value='14'
              size='sm'
              variant='ghost'
              color='blue-gray'
              className='rounded-full'
            />
          </ListItemSuffix>
        </ListItem>
        <ListItem>
          <ListItemPrefix>
            <UserCircleIcon className='h-5 w-5' />
          </ListItemPrefix>
          Profile
        </ListItem>
        <ListItem>
          <ListItemPrefix>
            <Cog6ToothIcon className='h-5 w-5' />
          </ListItemPrefix>
          Settings
        </ListItem>
        <ListItem>
          <ListItemPrefix>
            <PowerIcon className='h-5 w-5' />
          </ListItemPrefix>
          Log Out
        </ListItem>
      </List>
    </Card>
  )
}
