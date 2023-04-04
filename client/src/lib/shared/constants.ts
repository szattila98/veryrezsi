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

export const REQUIRED_VIOLATION_MSG = 'You need to fill this.';
export const MAX_LENGTH_VIOLATION_MSG = 'You need to choose a shorter one.';
export const EMAIL_VIOLATION_MSG = 'You need to type a valid email address.';
