#include "floui.h"

#define FLOUI_IMPL
#include "../floui/floui.hpp"


CFlouiViewController *CFlouiViewController_new(void *arg1, void *arg2, void *arg3) {
    return (CFlouiViewController *)new FlouiViewController(arg1, arg2, arg3);
}

CMainView *CMainView_new(const CFlouiViewController *fvc) {
    return (CMainView *)new MainView(*(FlouiViewController *)fvc, {});
}

void CMainView_add(CMainView *self, const CWidget *w) {
    ((MainView *)self)->add(*(Widget *)w);
}

CButton *CButton_new(const char *label) {
    return (CButton *)new Button(label);
}

void CButton_action(CButton *self, CFlouiCallback cb, void *data) {
    ((Button *)self)->action([=](Widget &) {
        (*cb)((CWidget *)self, data);
    });
}

CText *CText_new(const char *label) {
    return (CText *)new Text(label);
}

void CText_text(CText *self, const char *t) {
    ((Text *)self)->text(t);
}

void Cfloui_log(const char *s) {
    floui_log(s);
}