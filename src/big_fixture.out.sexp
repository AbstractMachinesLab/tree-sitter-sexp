(source_file
 (module_name (atom (unquoted_atom)))
 (module_export (atom (unquoted_atom)) (integer))
 (module_export (atom (unquoted_atom)) (integer))
 (module_export (atom (unquoted_atom)) (integer))
 (function_declaration
  (function_clause
   (atom (unquoted_atom))
   (lambda_clause
    (pattern (variable))
    (pattern (variable))
    (expression
     (function_call
      (qualified_function_name (expression (term (atom (unquoted_atom)))) (atom (unquoted_atom)))
      (expression
       (lambda
        (lambda_clause
         (pattern (variable))
         (pattern (variable))
         (expression
          (case
           (expression (variable))
           (case_clause
            (pattern (term (atom (unquoted_atom))))
            (expression
             (function_call
              (qualified_function_name
               (expression (term (atom (unquoted_atom)))) (atom (unquoted_atom)))
              (expression
               (lambda
                (lambda_clause
                 (pattern (variable))
                 (pattern (variable))
                 (expression
                  (case
                   (expression (variable))
                   (case_clause
                    (pattern (term (atom (unquoted_atom))))
                    (expression
                     (function_call
                      (qualified_function_name
                       (expression (term (atom (unquoted_atom))))
                       (atom (unquoted_atom)))
                      (expression
                       (lambda
                        (lambda_clause
                         (pattern (variable))
                         (pattern (variable))
                         (expression
                          (case
                           (expression (variable))
                           (case_clause
                            (pattern (term (atom (unquoted_atom))))
                            (expression
                             (case
                              (expression
                               (function_call
                                (qualified_function_name
                                 (expression
                                  (term (atom (unquoted_atom))))
                                 (atom (quoted_atom)))
                                (expression (variable))
                                (expression
                                 (function_call
                                  (qualified_function_name
                                   (expression
                                    (term
                                     (atom (unquoted_atom))))
                                   (atom (quoted_atom)))
                                  (expression
                                   (function_call
                                    (qualified_function_name
                                     (expression
                                      (term
                                       (atom (unquoted_atom))))
                                     (atom
                                      (quoted_atom)))
                                    (expression (variable))
                                    (expression (variable))))
                                  (expression (variable))))))
                              (case_clause
                               (pattern
                                (term (atom (unquoted_atom))))
                               (expression
                                (term
                                 (tuple
                                  (expression
                                   (term
                                    (atom (unquoted_atom))))
                                  (expression
                                   (function_call
                                    (qualified_function_name
                                     (expression
                                      (term
                                       (atom (unquoted_atom))))
                                     (atom
                                      (quoted_atom)))
                                    (expression
                                     (function_call
                                      (qualified_function_name
                                       (expression
                                        (term
                                         (atom
                                          (unquoted_atom))))
                                       (atom
                                        (quoted_atom)))
                                      (expression
                                       (variable))
                                      (expression (variable))))
                                    (expression (variable))))))))
                              (case_clause
                               (pattern
                                (term (atom (unquoted_atom))))
                               (expression
                                (term (atom (unquoted_atom))))))))
                           (case_clause
                            (pattern (variable))
                            (expression (variable))))))))
                      (expression (term (atom (unquoted_atom))))
                      (expression (variable)))))
                   (case_clause (pattern (variable)) (expression (variable))))))))
              (expression (term (atom (unquoted_atom))))
              (expression (variable)))))
           (case_clause (pattern (variable)) (expression (variable))))))))
      (expression (term (atom (unquoted_atom))))
      (expression (variable)))))))
 (function_declaration
  (function_clause
   (atom (unquoted_atom))
   (lambda_clause
    (pattern (variable))
    (pattern (variable))
    (expression
     (function_call
      (qualified_function_name (expression (term (atom (unquoted_atom)))) (atom (unquoted_atom)))
      (expression
       (lambda
        (lambda_clause
         (pattern (variable))
         (pattern (variable))
         (expression
          (case
           (expression (variable))
           (case_clause
            (pattern (term (atom (unquoted_atom))))
            (expression
             (function_call
              (qualified_function_name
               (expression (term (atom (unquoted_atom)))) (atom (unquoted_atom)))
              (expression
               (lambda
                (lambda_clause
                 (pattern (variable))
                 (pattern (variable))
                 (expression
                  (case
                   (expression (variable))
                   (case_clause
                    (pattern (term (atom (unquoted_atom))))
                    (expression
                     (case
                      (expression
                       (function_call
                        (qualified_function_name
                         (expression (term (atom (unquoted_atom))))
                         (atom (quoted_atom)))
                        (expression (variable))
                        (expression
                         (function_call
                          (qualified_function_name
                           (expression
                            (term (atom (unquoted_atom))))
                           (atom (quoted_atom)))
                          (expression (variable))
                          (expression (variable))))))
                      (case_clause
                       (pattern (term (atom (unquoted_atom))))
                       (expression
                        (term
                         (tuple
                          (expression (term (atom (unquoted_atom))))
                          (expression
                           (function_call
                            (qualified_function_name
                             (expression
                              (term (atom (unquoted_atom))))
                             (atom (quoted_atom)))
                            (expression (variable))
                            (expression (variable))))))))
                      (case_clause
                       (pattern (term (atom (unquoted_atom))))
                       (expression (term (atom (unquoted_atom))))))))
                   (case_clause (pattern (variable)) (expression (variable))))))))
              (expression (term (atom (unquoted_atom))))
              (expression (variable)))))
           (case_clause (pattern (variable)) (expression (variable))))))))
      (expression (term (atom (unquoted_atom))))
      (expression (variable)))))))
 (function_declaration
  (function_clause
   (atom (unquoted_atom))
   (lambda_clause
    (expression
     (term
      (list
       (expression
        (function_call
         (computed_function_name (expression (term (atom (unquoted_atom)))))
         (expression (term (integer)))
         (expression
          (term
           (list
            (expression (term (integer)))
            (expression
             (term
              (list (expression (term (integer)))
               (expression
                (term (list (expression (term (integer))) (expression (term (list))))))))))))))
       (expression
        (term
         (list
          (expression
           (function_call
            (computed_function_name (expression (term (atom (unquoted_atom)))))
            (expression (term (integer)))
            (expression
             (term
              (list
               (expression (term (integer)))
               (expression
                (term
                 (list
                  (expression (term (integer)))
                  (expression
                   (term
                    (list (expression (term (integer))) (expression (term (list))))))))))))))
          (expression (term (list)))))))))))))
