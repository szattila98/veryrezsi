import ctl from '@netlify/classnames-template-literals';

// COLORS
export const GRADIENT = ctl(`
    bg-gradient-to-r 
    from-blue-400   
    to-blue-700 
`);

// Validation
export const VALIDATION_MSG = ctl(`
    bg-red-500 
    py-1 
    px-2 
    text-white 
    rounded-b
`);

export const ALERT_MSG = ctl(`
    bg-red-500 
    py-1 
    px-2 
    text-center
    text-white 
    rounded
    mb-1
`);

export const TECHNICAL_ERROR_ALERT_MSG = 'Something went wrong on our side.';
export const REQUIRED_VIOLATION_MSG = 'You need to fill this.';
export const MAX_LENGTH_VIOLATION_MSG = 'You need to choose a shorter one.';
export const EMAIL_VIOLATION_MSG = 'You need to type a valid email address.';
export const UNSUCCESFUL_LOGIN_ALERT_MSG = 'Incorrect username or password.';
