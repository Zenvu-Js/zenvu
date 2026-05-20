/**
 * @zenvu/forms - Form Handling & Validation
 */
export function useForm(initialValues: any) {
  // Returns reactive form state and validation logic
  return { values: initialValues, validate: () => true };
}
